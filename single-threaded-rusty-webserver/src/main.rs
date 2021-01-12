use std::net::TcpListener; 
use std::net::TcpStream; 
use std::io::prelude::* ; 
use std::fs::File; 

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap(); 

    for stream in listener.incoming(){
        let stream = stream.unwrap(); 

        handle_connection(stream); 
    }
}


fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0;4096]; 

    stream.read(&mut buffer).unwrap(); 
    //println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n"; 

    let (status , file_name)  = if buffer.starts_with(get){
        ("HTTP/1.1 200 OK\r\n" , "hello.html")
    }
    else{
        ("HTTP/1.1 404 NOT FOUND\r\n" , "Error404.html")
    }; 

    let mut file = File::open(file_name).unwrap(); 

    let mut contents = String::new(); 

    file.read_to_string(&mut contents).unwrap(); 
    
    let response_headers = format!("{}{}" , status , "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n".to_string());

    let response = format!("{} {}" , response_headers , contents); 

    stream.write(response.as_bytes()).unwrap(); 
}