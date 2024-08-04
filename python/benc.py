def encrypt(data: bytes, key: bytes) -> bytes:
    res = []
    key_len = len(key)
    
    for i in range(len(data)):
        byte = data[i]
        key_byte = key[i % key_len]

        res[i] = (byte + key_byte) & 0xFF

    return res

def decrypt(data: bytes, key: bytes) -> bytes:
    res = []
    key_len = len(key)
    
    for i in range(len(data)):
        byte = data[i]
        key_byte = key[i % key_len]

        res[i] = (byte - key_byte) & 0xFF

    return res

if __name__ == "__main__":
    print("This script is meant to be used a library.")
    print("use `import benc` to access its functions.")