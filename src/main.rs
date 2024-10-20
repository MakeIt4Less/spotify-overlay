use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

mod spotify;
mod login;

fn main() -> Result<(),std::io::Error> {
    // spotify::SpotifyController::new();
    let listener = TcpListener::bind("127.0.0.1:6969")?;
    for stream in listener.incoming() {
        let stream = stream?;
        handle_connection(stream);
    }
    Ok(())
}

fn handle_connection(stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let mut http_request:Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    for line in &http_request {
        println!("{line}")
    }
    println!("");
    if http_request.len() <= 0 {
        return;
    }
    let request_line = http_request.remove(0);
    let request_info = request_line.split(" ").nth(1).unwrap();
    let (request_dest,rest_of_request) = request_info.split_once("?").unwrap_or((request_info,""));
    match request_dest {
        "/" => handle_root(stream,rest_of_request),
        "/overlay" => todo!("impl overlay"),
        _ => eprintln!("404 Not found:{request_dest}")
    }
}

fn handle_root(mut stream:TcpStream,dest:&str) {
    if let Some(index) = dest.find("code="){
        println!("ind: {index}, substr: {}",dest[index+5..index+16].to_string());
        let spot_cont = spotify::SpotifyController::new(dest[index+5..].to_string());
        println!("Listening to: {}",spot_cont.get_current_song());
    }
    let status_line = "HTTP/1.1 200 OK";
    let contents = login::get_login_html();
    
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
