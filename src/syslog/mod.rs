use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::thread;

pub fn start_server() {
    thread::spawn(|| {
        // Probably handle this error in more forgiving way?
        let mut socket = UdpSocket::bind("0.0.0.0:8514").expect("Failed to start syslog server");
        let mut buffer: Vec<u8> = vec![0; 480];
        loop {
            buffer.clear();
            buffer.resize(480, 0);
            let (read, source) = socket.recv_from(buffer.as_mut()).unwrap();
            let message_string = String::from_utf8(buffer[0..read].to_vec())
                .unwrap_or("error INVALID MESSAGE".to_string());
            let s: Vec<&str> = message_string.splitn(2, ' ').collect();
            if s.len() != 2 {
                println!("Invalid message received from {}", source);
            } else {
                let topics: Vec<&str> = s[0].split(',').collect();
                // For now we just log it to the console
                println!("{}: {:?} - {}", source.ip(), topics, s[1]);
            }
        }
    });
}
