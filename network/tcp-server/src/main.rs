use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::thread;
use std::str;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512]; // 创建一个缓冲区存放来自客户端的数据

    loop {
        match stream.read(&mut buffer) {
            Ok(size) => {
                // 当接收到数据时，打印数据并发送回一个确认消息
                if size == 0 { // 如果没有数据，断开连接
                    break;
                }
                println!("Received: {}", str::from_utf8(&buffer[..size]).unwrap());
                stream.write(&buffer[..size]).unwrap(); // 将接收到的数据回送给客户端
            },
            Err(e) => {
                println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
                println!("Error: {}", e);
                break;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // 绑定地址和端口
    println!("Server listening on port 7878");

    let x = 6;
    let y = 10;
    let result = if x > y { x } else { y };
    println!("The greater number is {}", result);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(|| {
                    // 使用线程处理每个客户端的数据
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    drop(listener); // 一旦服务器停止接受新的连接，关闭监听器
}
