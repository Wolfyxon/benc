window.addEventListener("load", () => {
    const textEncoder = new TextEncoder();
    const textDecoder = new TextDecoder();

    /*
    const text = {
        btnEncrypt: document.getElementById("btn-encrypt-text"),
        btnDecrypt: document.getElementById("btn-decrypt-text"),
        input:      document.getElementById("textarea"),
        password:   document.getElementById("text-password")
    };


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

    const fileReader = new FileReader();

    const file = {
        btnEncrypt: document.getElementById("btn-encrypt-file"),
        btnDecrypt: document.getElementById("btn-decrypt-file"),
        input:      document.getElementById("file-input"),
        password:   document.getElementById("file-password")
    };

    function downloadBuffer(buffer, fileName) {
        const blob = new Blob([buffer]);
        
        const a = document.createElement("a");
        a.href = URL.createObjectURL(blob);
        a.download = fileName;
        a.click();
    }

    file.btnEncrypt.addEventListener("click", () => {
        const f = file.input.files[0];
        if(!f) return;

        fileReader.readAsArrayBuffer(f);

        fileReader.onload = () => {
            const data = new Uint8Array(fileReader.result);
            const encrypted = benc.encrypt(data, textEncoder.encode(file.password.value));
            downloadBuffer(encrypted, f.name);
        }
    });

    file.btnDecrypt.addEventListener("click", () => {
        const f = file.input.files[0];
        if(!f) return;

        fileReader.readAsArrayBuffer(f);

        fileReader.onload = () => {
            const data = new Uint8Array(fileReader.result);
            const decrypted = benc.decrypt(data, textEncoder.encode(file.password.value));
            downloadBuffer(decrypted, f.name);
        }
    });

});