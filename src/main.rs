use std::fs::read_to_string;
use std::net::TcpListener; // For binding with socket of the server
use std::net::TcpStream; // For taking data in form of streams from the server
use std::io::prelude::*;
use std::thread;
use std::time::Duration; // for importing all required i/o operations

// Tcp/IP layer: physical data-link internet transport application
// Applcation: http ftp smtp // for presentatin, formating data encryption and start or stop server 
// transport: Tcp UDP // for error handling and sequencing also add port numbers
// Network: IP routers // source and destination ip address  
// Data link: ethernet switches // address is added
// Physical: cables // for carrying data
// Data -> Tcp(DATA){Segment} -> IP(TCP(DATA)) {Packet} -> (ETHERNET)(IP(TCP(DATA)))(ETHERNET) {Frame}
// TCP -> source and destincation port number 
// IP  -> source and destiation ip address 


// TCP contains 3 numbers: acknowlegment numbers, squencing and checksum
//It relies on syn and sync ack send and response where one sends synchronous data and get synchronous data with async and get acknowleged
// then it checks checksum if checksum matches it's good otherwise segment will be disregarded


fn main(){
    let listener = 
        TcpListener::bind("127.0.0.1:7878").unwrap(); // Binding the connection wth socket

    for stream in listener.incoming() { // Iterate through all the incoming connection and deal with it
        let stream = stream.unwrap(); // Unwraping the stream to work on it should be error handled too but not doing it here
        handle_connection(stream); 
    }
}

fn handle_connection(mut stream: TcpStream)
{
    let mut buff = [0; 1024]; // Take slice of bytes

    stream.read(&mut buff).unwrap(); // Read the bytes into buff

    let get = b"GET / HTTP/1.1\r\n"; // Status code in bytes
    let sleep = b"GET /sleep HHTP/1.1 \r\n"; // For handling if one request takes a little long time

    let (status_code, contents) =
        if buff.starts_with(get){
            ("HTTP/1.1 200 OK", read_to_string("../index.html").unwrap())
        }
        else if buff.starts_with(sleep){
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", read_to_string("../index.html").unwrap())
        }
        else{
            ("HTTP/1.1 404 NOT FOUND", read_to_string("../404.html").unwrap())
        };

    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_code, contents.len(), contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}