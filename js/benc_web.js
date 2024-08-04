// JavaScript BENC version for web browsers
// Accessed by importing the script, then using the functions and classes in other scripts

const benc = {};

/**
 * Encrypts bytes
 * @param {number[]} data 
 * @param {number[]} key
 * @returns {number[]}
 */
benc.encrypt = function(data, key) {
    const res = [];

    for(let i = 0; i < data.length; i++) {
        const byte = data[i];
        const keyByte = key[i % key.length];

        res.push(byte + keyByte);
    }

    return res;
}

/**
 * Decrypts bytes
 * @param {number[]} encrypted 
 * @param {number[]} key
 * @returns {number[]}
 */
benc.decrypt = function(encrypted, key) {
    const res = [];

    for(let i = 0; i < encrypted.length; i++) {
        const byte = encrypted[i];
        const keyByte = key[i % key.length];

        res.push(byte - keyByte);
    }

    return res;
}