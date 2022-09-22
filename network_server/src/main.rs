use std::net::{ TcpListener, TcpStream};
use std::{
    io::{self, Read, Write},
    thread, time,
    str,
};
fn handle_client(mut steam: TcpStream) -> io::Result<()> {
    
    for _ in 0..=1000 {
        let mut buf = [0; 512];
        match steam.read(&mut buf) {
            Ok(_) => {
                let recvstr = str::from_utf8(&buf).expect("取值失败").to_string();
                let mut recvstr = recvstr.replace("\n", "").replace("\0", "");
                recvstr.push_str("--来自服务端的回复\n");
                println!("回复客户端请求:{:#?}",recvstr);
                steam.write(&recvstr.as_bytes()).expect("写错误");
            }
            Err(e) => {
                return Err(e); 
            }
        }
        thread::sleep(time::Duration::from_secs(1));
    }
    Ok(())
}
fn main() {
    println!("start");
    listener();
}
fn listener() {
    let addr = "127.0.0.1:8000"; //因为&str实现了ToSocketAddrs Trait 所以bind方法的参数可以使用&str
    if let Ok(listener) = TcpListener::bind(addr) {
        let mut handles = Vec::new();
        for stream in listener.incoming() {
            println!("新链接进入");
            let st = stream.unwrap();
            handles.push(thread::spawn(move || {
                handle_client(st).unwrap_or_else(|error| eprintln!("{:?}", error));
            }));
        }
        for handle in handles {
            handle.join().unwrap();
        }
    } else {
        println!("侦听失败！");
    }
    println!("over");
}
