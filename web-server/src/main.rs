use std::{net::{TcpListener, TcpStream}, io::{Read, Write}};
use std::fs;

fn main() {
    // 监听地址和端口
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // 遍历监听进入的流
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

// 链接处理函数
fn handle_connection(mut stream: TcpStream) {
    // 定义空数组
    let mut buffer = [0;512];
    // 定义get字符串
    let get = b"GET / HTTP/1.1\r\n";
    // 判断请求 是根目录 走hello
    let (status_line, filename) = if buffer.starts_with(get){
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line ,contents);
    // 将返回信息写入stream
    stream.write(response.as_bytes()).unwrap();
    // 阻塞等待写入完成
    stream.flush().unwrap();

}
