pub fn encrypt(byte_vec: Vec<u8>, key_byte_vec: Vec<u8>) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    let key_len = key_byte_vec.len();

    for i in 0..byte_vec.len() {
        let byte = byte_vec[i];
        let offset = key_byte_vec[i % key_len];

        res.push(byte.wrapping_add(offset));
    }

    return res;
}

pub fn decrypt(byte_vec: Vec<u8>, key_byte_vec: Vec<u8>) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    let key_len = key_byte_vec.len();

    for i in 0..byte_vec.len() {
        let byte = byte_vec[i];
        let offset = key_byte_vec[i % key_len];

        res.push(byte.wrapping_sub(offset));
    }

    return res;
}

pub unsafe fn encrypt_string(str: String, password: String) -> String {
    let bytes = encrypt(str.as_bytes().to_vec(), password.as_bytes().to_vec());

    return String::from_utf8_unchecked(bytes);
}

pub unsafe fn decrypt_string(str: String, password: String) -> String {
    let bytes = decrypt(str.as_bytes().to_vec(), password.as_bytes().to_vec());

    return String::from_utf8_unchecked(bytes);
}