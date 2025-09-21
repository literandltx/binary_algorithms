#[cfg(test)]
mod tests {
    use base64::{Engine as _, engine::general_purpose};

    use crate::lab1::base64::{decode, encode};

    #[test]
    fn test_encode_matches_lib() {
        let data: &[u8] = b"Hello, custom Base64!";
        let custom: String = encode(data);
        let lib: String = general_purpose::STANDARD.encode(data);

        assert_eq!(
            custom, lib,
            "Custom Base64 encode does not match library encode"
        );
    }

    #[test]
    fn test_decode_matches_original() {
        let data: &[u8] = b"Some test string with numbers 12345";
        let encoded: String = encode(data);
        let decoded: Vec<u8> = decode(&encoded).expect("decode failed");

        assert_eq!(decoded, data);
    }

    #[test]
    fn demo_test() {
        let data: &[u8] = b"Hello, Rust! The original codes are implemented on a private codebase and will not be released. Hello, Rust! The original codes are implemented on a private codebase and will not be released. Hello, Rust! The original codes are implemented on a private codebase and will not be released.";
        let encoded: String = encode(data);
        let encoded_lib: String = general_purpose::STANDARD.encode(data);
        let decoded: Vec<u8> = decode(&encoded).unwrap();
        let decoded_lib: Vec<u8> = general_purpose::STANDARD.decode(&encoded).unwrap();

        assert_eq!(encoded_lib, encoded);
        assert_eq!(
            String::from_utf8_lossy(&decoded),
            String::from_utf8_lossy(&decoded_lib)
        );
    }
}
