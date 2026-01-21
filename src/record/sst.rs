use super::Data as ContinueData;
use crate::error::Result;
use binrw::{
    BinRead,
    helpers::until,
};
use encoding_rs::Encoding;

// 2.4.265
#[derive(Debug, BinRead)]
pub struct Data {
    _len: u16,

    _total: i32,
    _unique: i32,

    #[br(count = _len - 8)]
    bytes: Vec<u8>,

    #[br(parse_with = until(|cd: &ContinueData| cd.r#type != 0x003C))]
    continues: Vec<ContinueData>,

    #[br(ignore)]
    pub strs: Vec<String>,
}

/// Helper struct to track position across SST record and Continue records
struct SstReader<'a> {
    /// Main SST record bytes (after header)
    main_bytes: &'a [u8],
    /// Continue record bytes (each with grbit prefix for string continuations)
    continue_records: Vec<&'a [u8]>,
    /// Current position in main_bytes or continue_records
    main_offset: usize,
    /// Current continue record index (-1 = still in main_bytes)
    continue_idx: i32,
    /// Offset within current continue record
    continue_offset: usize,
    /// Whether we're in the middle of a string (affects grbit handling)
    in_string: bool,
    /// Current string's hbyte flag (for Continue record handling)
    current_hbyte: bool,
}

impl<'a> SstReader<'a> {
    fn new(main_bytes: &'a [u8], continues: &'a [ContinueData]) -> Self {
        SstReader {
            main_bytes,
            continue_records: continues.iter().map(|c| c.bytes.as_slice()).collect(),
            main_offset: 0,
            continue_idx: -1,
            continue_offset: 0,
            in_string: false,
            current_hbyte: true,
        }
    }

    fn remaining(&self) -> usize {
        if self.continue_idx < 0 {
            let main_remaining = self.main_bytes.len().saturating_sub(self.main_offset);
            let continue_total: usize = self.continue_records.iter().map(|c| c.len()).sum();
            main_remaining + continue_total
        } else {
            let idx = self.continue_idx as usize;
            if idx >= self.continue_records.len() {
                return 0;
            }
            let current_remaining = self.continue_records[idx].len().saturating_sub(self.continue_offset);
            let future_total: usize = self.continue_records[idx + 1..].iter().map(|c| c.len()).sum();
            current_remaining + future_total
        }
    }

    /// Read a single byte, handling Continue record transitions
    fn read_byte(&mut self) -> Option<u8> {
        if self.continue_idx < 0 {
            // Still in main SST record
            if self.main_offset < self.main_bytes.len() {
                let b = self.main_bytes[self.main_offset];
                self.main_offset += 1;
                return Some(b);
            }
            // Move to first continue record
            if !self.continue_records.is_empty() {
                self.continue_idx = 0;
                self.continue_offset = 0;
                // If we're in the middle of a string, skip the grbit byte
                if self.in_string && !self.continue_records[0].is_empty() {
                    let grbit = self.continue_records[0][0];
                    self.current_hbyte = (grbit & 0x01) == 0x00;
                    self.continue_offset = 1;
                }
                return self.read_byte();
            }
            return None;
        }

        let idx = self.continue_idx as usize;
        if idx >= self.continue_records.len() {
            return None;
        }

        if self.continue_offset < self.continue_records[idx].len() {
            let b = self.continue_records[idx][self.continue_offset];
            self.continue_offset += 1;
            return Some(b);
        }

        // Move to next continue record
        self.continue_idx += 1;
        self.continue_offset = 0;
        let next_idx = self.continue_idx as usize;
        if next_idx < self.continue_records.len() {
            // If we're in the middle of a string, skip the grbit byte
            if self.in_string && !self.continue_records[next_idx].is_empty() {
                let grbit = self.continue_records[next_idx][0];
                self.current_hbyte = (grbit & 0x01) == 0x00;
                self.continue_offset = 1;
            }
            return self.read_byte();
        }
        None
    }

    /// Read a u16 (little endian)
    fn read_u16(&mut self) -> Option<u16> {
        let lo = self.read_byte()? as u16;
        let hi = self.read_byte()? as u16;
        Some(lo | (hi << 8))
    }

    /// Read an i32 (little endian)
    fn read_i32(&mut self) -> Option<i32> {
        let b0 = self.read_byte()? as i32;
        let b1 = self.read_byte()? as i32;
        let b2 = self.read_byte()? as i32;
        let b3 = self.read_byte()? as i32;
        Some(b0 | (b1 << 8) | (b2 << 16) | (b3 << 24))
    }

    /// Mark that we're starting to read string bytes
    fn start_string(&mut self, hbyte: bool) {
        self.in_string = true;
        self.current_hbyte = hbyte;
    }

    /// Mark that we're done reading string bytes
    fn end_string(&mut self) {
        self.in_string = false;
    }

    /// Read string bytes with proper Continue handling
    /// This handles the case where the encoding might change at Continue boundaries
    fn read_string_bytes(&mut self, cch: u16, initial_hbyte: bool) -> Option<Vec<u8>> {
        self.start_string(initial_hbyte);

        // We need to read cch characters
        // If hbyte (high-byte compression), each char is 1 byte
        // If not hbyte, each char is 2 bytes (UTF-16LE)
        // The encoding can change at Continue boundaries

        let mut result = Vec::new();
        let mut chars_read = 0u16;
        let mut current_hbyte = initial_hbyte;

        while chars_read < cch {
            // Check if we're about to cross a Continue boundary
            let bytes_available = self.bytes_until_boundary();

            // If at boundary (no bytes available), trigger transition by reading a byte
            // The read_byte function will handle Continue record transition and grbit
            if bytes_available == 0 {
                // Force transition to next Continue record
                // This read will skip grbit if we're in a string
                if current_hbyte {
                    let b = self.read_byte()?;
                    result.push(b);
                    result.push(0x00);
                    chars_read += 1;
                } else {
                    let lo = self.read_byte()?;
                    let hi = self.read_byte()?;
                    result.push(lo);
                    result.push(hi);
                    chars_read += 1;
                }
                // Update encoding from grbit (read_byte updated current_hbyte)
                current_hbyte = self.current_hbyte;
                continue;
            }

            if current_hbyte {
                // High-byte compression: 1 byte per char, expand to 2 bytes
                let chars_available = bytes_available as u16;
                let chars_to_read = std::cmp::min(cch - chars_read, chars_available);

                for _ in 0..chars_to_read {
                    let b = self.read_byte()?;
                    result.push(b);
                    result.push(0x00); // Expand to UTF-16LE
                }
                chars_read += chars_to_read;
            } else {
                // UTF-16LE: 2 bytes per char
                let chars_available = (bytes_available / 2) as u16;
                let chars_to_read = std::cmp::min(cch - chars_read, chars_available);

                for _ in 0..chars_to_read {
                    let lo = self.read_byte()?;
                    let hi = self.read_byte()?;
                    result.push(lo);
                    result.push(hi);
                }
                chars_read += chars_to_read;
            }

            // If we still need more characters, we've hit a Continue boundary
            // The read_byte function handles skipping grbit and updating current_hbyte
            if chars_read < cch {
                current_hbyte = self.current_hbyte;
            }
        }

        self.end_string();
        Some(result)
    }

    /// Returns bytes available before hitting a Continue boundary
    fn bytes_until_boundary(&self) -> usize {
        if self.continue_idx < 0 {
            self.main_bytes.len().saturating_sub(self.main_offset)
        } else {
            let idx = self.continue_idx as usize;
            if idx >= self.continue_records.len() {
                0
            } else {
                self.continue_records[idx].len().saturating_sub(self.continue_offset)
            }
        }
    }
}

impl Data {
    pub fn decode(&mut self, _encoding: &'static Encoding) -> Result<()> {
        let mut reader = SstReader::new(&self.bytes, &self.continues);
        let unique = self._unique as usize;

        self.strs = Vec::with_capacity(unique);

        for _ in 0..unique {
            if reader.remaining() < 3 {
                // Not enough data for string header
                break;
            }

            // Read XLUnicodeRichExtendedString header
            let cch = match reader.read_u16() {
                Some(v) => v,
                None => break,
            };

            let flags = match reader.read_byte() {
                Some(v) => v,
                None => break,
            };

            let hbyte = (flags & 0x01) == 0x00;  // High-byte compression
            let hext = (flags >> 2) & 0x01 == 0x01;  // Extended string
            let hrun = (flags >> 3) & 0x01 == 0x01;  // Rich string

            // Read optional cRun (formatting runs count)
            let crun = if hrun {
                reader.read_u16().unwrap_or(0)
            } else {
                0
            };

            // Read optional cbExtRst (extended string size)
            let cext = if hext {
                reader.read_i32().unwrap_or(0)
            } else {
                0
            };

            // Read string bytes with proper Continue handling
            let str_bytes = match reader.read_string_bytes(cch, hbyte) {
                Some(b) => b,
                None => break,
            };

            // Skip formatting runs (4 bytes each: ich:u16, ifnt:u16)
            for _ in 0..crun {
                reader.read_u16();
                reader.read_u16();
            }

            // Skip extended data
            for _ in 0..cext {
                reader.read_byte();
            }

            // Decode the string (always UTF-16LE after expansion)
            let s = encoding_rs::UTF_16LE.decode(&str_bytes).0.to_string();
            self.strs.push(s);
        }

        Ok(())
    }
}
