use std::net::UdpSocket;
use std::io::Result;
fn main() ->Result<()>{
    let socket = UdpSocket::bind("127.0.0.1:8080")?;
    loop{
        let mut buffer = [0u8;1500];
        let (amt,addr) = socket.recv_from(&mut buffer)?;
        let mut buffer = &mut buffer[..amt];
        buffer.reverse();
        socket.send_to(buffer, addr)?;
    }
}
