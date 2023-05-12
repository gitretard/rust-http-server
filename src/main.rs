mod lib;
use std::{
    net::{TcpListener}
};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let file = match lib::load_file("README.md"){
        Ok(a) => a,
        Err(a) => {println!("{}",a);return},
    };
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let mut n = match lib::parsereq(stream,true){
            Ok(egr) => {egr},
            Err(_) => {return;}
        };
        let mut r = n.new_response();
        r.Body = file.clone();
        r.HttpVer = "HTTP/1.1".to_string();
        r.Status = "200 OK".to_string();
        r.insert_Header("Content-Type", "text/plain");
        r.done();
    }
}