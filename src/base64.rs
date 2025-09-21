const BASE64_ALPHABET: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+\\";

pub(crate) fn encode(input: &[u8]) -> String {
    let mut out: String = String::new();
    let mut i: usize = 0;

    while i < input.len() {
        let chunk: &[u8] = &input[i..usize::min(i + 3, input.len())];
        let mut buf: u32 = 0;

        for &b in chunk {
            buf = (buf << 8) | b as u32;
        }

        let padding: usize = 3 - chunk.len();
        buf <<= padding * 8;

        for j in (0..4).rev() {
            let idx = ((buf >> (6 * j)) & 0x3F) as usize;
            out.push(BASE64_ALPHABET[idx] as char);
        }

        if padding > 0 {
            let len: usize = out.len();
            for k in 0..padding {
                out.replace_range(len - 1 - k..len - k, "=");
            }
        }

        i += 3;
    }

    out
}

pub(crate) fn decode(encoded: &str) -> Result<Vec<u8>, String> {
    let mut out: Vec<u8> = Vec::new();
    let mut buf: u32 = 0;
    let mut bits_collected = 0;

    for c in encoded.chars() {
        if c == '=' {
            break;
        }

        let val = BASE64_ALPHABET
            .iter()
            .position(|&b| b as char == c)
            .ok_or_else(|| format!("Invalid character: {}", c))?;

        buf = (buf << 6) | val as u32;
        bits_collected += 6;

        if bits_collected >= 8 {
            bits_collected -= 8;
            let byte: u32 = (buf >> bits_collected) & 0xFF;
            out.push(byte as u8);
        }
    }

    Ok(out)
}
