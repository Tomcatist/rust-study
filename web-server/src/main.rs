use std::{net::{TcpListener, TcpStream}, io::{Read, Write}, thread};
use std::fs;
use std::time::Duration;
use web_server::ThreadPool;

fn main() {
    // 监听地址和端口
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // 创建线程池
    let pool = ThreadPool::new(4);
    // 遍历监听进入的流
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        // 用线程池执行
        pool.execute(|| {
            handle_connection(stream);
        });

        println!("Shuting down!");

    }
}

// 链接处理函数
fn handle_connection(mut stream: TcpStream) {
    // 定义空数组
    let mut buffer = [0;512];
    // 读取请求并放入数组
    stream.read(&mut buffer).unwrap();
    // 定义get字符串
    let get = b"GET / HTTP/1.1\r\n";
    // 模拟长时间响应
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    // 判断请求 是根目录 走hello 其他走404  /sleep模拟长时间响应
    let (status_line, filename) = if buffer.starts_with(get){
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    }else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    }
    else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line ,contents);
    // 将返回信息写入stream
    stream.write(response.as_bytes()).unwrap();
    // 阻塞等待写入完成
    stream.flush().unwrap();

}
