use windows_sys::Win32::System::Diagnostics::ToolHelp::{CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32FirstW, Process32NextW };
use windows_sys::Win32::Foundation::GetLastError;

pub fn ps() -> String {
    let mut result = String::new();
    result.push_str("PID\tPPID\tProcess Name\n");

    unsafe{
        let snapshot =  CreateToolhelp32Snapshot(2, 0);

        let mut process_entry: PROCESSENTRY32W = std::mem::zeroed();
        process_entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;

        let success = Process32FirstW(snapshot, &mut process_entry);
        if success != 1 {
            return "".to_string()
        }

        loop {
            let process_id = process_entry.th32ProcessID;
            let process_name: Vec<u16> = process_entry.szExeFile[..]
            .iter()
            .take_while(|&&c| c != 0)
            .cloned()
            .collect();
            let process_name_string = String::from_utf16_lossy(&process_name);

            result.push_str(&format!("{}\t{}\t{}\n", process_id, process_entry.th32ParentProcessID, process_name_string));

            let success = Process32NextW(snapshot, &mut process_entry);
            if success != 1 {
                let last_error = GetLastError();
                if last_error != 0x12B {
                    break;
                }
                break;
            }
        }

    }

    result
}