use std::io;
use std::net::TcpStream;
mod stream;

fn main(){
    println!("run the main function");
    let mut stream = TcpStream::connect("8080");
}