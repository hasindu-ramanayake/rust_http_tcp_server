use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;

use server::ThreadPool;


fn main() {
    
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    // creating a threadPool;
    let thread_pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let strm = stream.unwrap();
        thread_pool.execute( || {
            handle_connection(strm);
        });
    }
}


fn handle_connection(mut stream: TcpStream) {
    let mut buff: [u8; 1024] = [0; 1024];
    stream.read(&mut buff).unwrap();
    
    let get: &[u8; 16] = b"GET / HTTP/1.1\r\n";
    
    let (status,file_name) = if buff.starts_with(get) {
        ( "HTTP/1.1 200 OK", "index.html")
    } else {
        ( "HTTP/1.1 404 NOT FOUND", "404.html")
    };


    let content = fs::read_to_string(file_name).unwrap();
    let res = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        content.len(),
        content
    );

    stream.write(res.as_bytes()).unwrap();
    stream.flush().unwrap();
    

}
