use std::io::Read;

fn main() {
    let mut f = cfb::open("tests/test_data/DateFormats.xls").unwrap();
    let mut stream = f.open_stream("/Workbook").unwrap();

    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).unwrap();

    println!("Workbook stream size: {} bytes (0x{:x})", buf.len(), buf.len());

    // Check last few bytes
    let start = buf.len().saturating_sub(20);
    println!("\nLast 20 bytes starting at offset 0x{:x}:", start);
    for (i, b) in buf[start..].iter().enumerate() {
        println!("  0x{:03x}: 0x{:02x} ({})", start + i, b, b);
    }

    // Check around 0xffd
    if buf.len() > 0xffd {
        println!("\nBytes around 0xffd:");
        let start_off = 0xff8;
        let end_off = (0x1004).min(buf.len());
        for i in start_off..end_off {
            if i < buf.len() {
                println!("  0x{:03x}: 0x{:02x}", i, buf[i]);
            } else {
                println!("  0x{:03x}: <EOF>", i);
            }
        }
    }

    // Find EOF records
    println!("\nLooking for EOF records (0x0A 0x00):");
    for i in 0..buf.len()-1 {
        if buf[i] == 0x0A && buf[i+1] == 0x00 {
            // Check if next 2 bytes are length (should be 0x00 0x00 for EOF)
            if i + 3 < buf.len() && buf[i+2] == 0x00 && buf[i+3] == 0x00 {
                println!("  Found EOF record at offset 0x{:x}", i);
            }
        }
    }
}
