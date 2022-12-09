use std::{net::{TcpListener, TcpStream}, io::{Read, BufReader, BufRead, Write}, fs, time::Duration, thread};
use server::ThreadPool;

fn main() {
    // 透過 TcpListener，我們可以監聽 127.0.0.1:7878 位址上的 TCP 連線
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // 创建一个长度为4的线程池
    let pool = ThreadPool::new(4);

    // TcpListener 的 incoming 方法會回傳一個疊代器，給予我們一連串的流
    for stream in listener.incoming().take(2) {
        // 一個流代表的是客戶端與伺服器之間的開啟的連線
        let stream = stream.unwrap();
        println!("建立连接");

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("two request")
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader
        .lines()
        .next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}