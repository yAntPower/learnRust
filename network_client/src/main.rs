use std::{
    io::{self, prelude::*, BufReader, Write},
    net::TcpStream,
    //str,
};
fn main() {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8000") {
        for _ in 0..10 {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("控制台输入错误");
            stream.write(input.as_bytes()).expect("send message failed");

            let mut reader = BufReader::new(&stream);
            let mut string = String::new();
            //可能是因为没有结束标志EOF 导致方法无法执行完毕
            //reader.read_to_string(&mut string).expect("客户端读取服务器回复失败"); 
            reader.read_line(&mut string).expect("客户端读取服务器回复失败");
            println!("服务器端回复:{}", string);
            
            //下面的方式也可以读
            // let mut buf:Vec<u8> = Vec::new();
            // reader.read_until(b'\n', &mut buf).expect("读取服务器回复失败");
            // println!("客户端接收：{}",str::from_utf8(&buf).unwrap());
        }
    } else {
        println!("连接服务端失败");
    }
}
