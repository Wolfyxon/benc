// JavaScript BENC version for web browsers
// Accessed by importing the script, then using the functions and classes in other scripts

const benc = {};

/**
 * Encrypts bytes
 * @param {Uint8Array} data 
 * @param {Uint8Array} key
 * @returns {Uint8Array}
 */
benc.encrypt = function(data, key) {
    const res = new Uint8Array(data.length);

    for(let i = 0; i < data.length; i++) {
        const byte = data[i];
        const keyByte = key[i % key.length];

        res[i] = byte + keyByte;
    }

    return res;
}

/**
 * Decrypts bytes
 * @param {Uint8Array} encrypted 
 * @param {Uint8Array} key
 * @returns {Uint8Array}
 */
benc.decrypt = function(encrypted, key) {
    const res = new Uint8Array(encrypted.length);

    for(let i = 0; i < encrypted.length; i++) {
        const byte = encrypted[i];
        const keyByte = key[i % key.length];

        res[i] = byte - keyByte;
    }

    return res;
}