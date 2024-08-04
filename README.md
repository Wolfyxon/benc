# BENC - Basic Encryption
Basic weak encryption algorithm I made because I was bored. 

An [online demo](https://wolfyxon.github.io/benc/js/demo) is available, but currently raw text encryption/decryption is not supported. 

## Example CLI usages
### Encrypting a message
```
benc encrypt_text Would be funny if you could read this  
Enter password: jA93lkX82aR
```
### Decrypting a file
```
benc decrypt_file minecraft2.jar th3rsJu$t-no-W@Y
``` 

## How it works?
### Encryption
There are 2 sets of bytes. Input bytes and key bytes (message and password).
Each key byte is added to the input byte at the same index. If the input index is bigger than the key size, it will wrap.

```rs
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
```

### Decryption
Basically the same as encryption, but the values are subtracted instead of adding them.

```rs
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
```
