use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};
use unit20_web::ThreadPool;
fn main() {
    let addr = String::from("127.0.0.1:7878");
    let listener = TcpListener::bind(addr).unwrap();
    let pool = ThreadPool::new(2);
    for conn in listener.incoming() {
        let conn = conn.unwrap();
        pool.excute(||{
            handle_connection1(conn);
        });
       
    }
}
fn handle_connection1(mut conn: TcpStream) {

    let mut buf = [0; 1024];
    conn.read(&mut buf).unwrap();
    let get = b"GET / HTTP/1.1\r\n";

    let (statu, filename) = if buf.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        statu,
        contents.len(),
        contents
    );
    conn.write(response.as_bytes()).unwrap();
    conn.flush().unwrap();
}
