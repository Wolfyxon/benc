window.addEventListener("load", () => {
    /*
    const text = {
        btnEncrypt: document.getElementById("btn-encrypt-text"),
        btnDecrypt: document.getElementById("btn-decrypt-text"),
        input:      document.getElementById("textarea"),
        password:   document.getElementById("text-password")
    };

    const textEncoder = new TextEncoder();
    const textDecoder = new TextDecoder();

    text.btnEncrypt.addEventListener("click", () => {
        const value = textEncoder.encode(text.input.value);
        const key = textEncoder.encode(text.password.value);
        const encrypted = benc.encrypt(value, key);

        text.input.value = textDecoder.decode(encrypted);
    });

    text.btnDecrypt.addEventListener("click", () => {
        const value = textEncoder.encode(text.input.value);

        const key = textEncoder.encode(text.password.value);
        const decrypted = benc.decrypt(value, key);

        text.input.value = textDecoder.decode(decrypted);
    });*/

    const file = {
        btnEncrypt: document.getElementById("btn-encrypt-file"),
        btnDecrypt: document.getElementById("btn-decrypt-file"),
        input:      document.getElementById("file-input"),
        password:   document.getElementById("file-password")
    };

});