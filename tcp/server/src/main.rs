use std::net::{TcpListener,TcpStream};
use std::thread::{self, JoinHandle};
use std::io::{self, Read,Write};
use std::time;

fn handle_client(mut stream: TcpStream)->io::Result<()>{
    let mut msg = [0;512];
    let read_len = stream.read(&mut msg)?;
    if read_len == 0 {
        return Ok(());
    }
    stream.write(&msg[..read_len])?;
    thread::sleep(time::Duration::from_secs(2));
    Ok(())
}

fn main() -> std::io::Result<()>{
    let listner = TcpListener::bind("127.0.0.1:8080")?;
    let mut thread_vec:Vec<JoinHandle<()>> = vec![];
    for stream in listner.incoming() {
        let stream = stream.expect("failed to connect");
        let handle = thread::spawn(move || {
            handle_client(stream).unwrap_or_else(|error|{eprintln!("{:?}", error)});
        });
        thread_vec.push(handle);
    }
    for handle in thread_vec {
        handle.join();
    }
    Ok(())
}
