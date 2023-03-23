use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::thread;

pub struct TCPPeer {
    pub outgoing: bool,
}

pub struct TCPTransport<'a> {
    listener_addr: &'a str,
}

pub fn new_TCPTransport(listener_addr: &str) -> TCPTransport {
    TCPTransport { listener_addr }
}

impl TCPTransport<'_> {
    pub fn start(&self) {
        let listener = TcpListener::bind("localhost".to_string() + self.listener_addr).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Write;
    use std::sync::Arc;
    use std::time;

    #[test]
    fn test_tcp() {
        let tr = new_TCPTransport(&":3000");
        thread::spawn(move || {
            //tr.start();
            loop {
                tr.start();
            }
        });
        thread::sleep(time::Duration::from_millis(1000));
        let mut stream = TcpStream::connect("localhost:3000").unwrap();
        stream.write("hello world".as_bytes()).unwrap();
    }
}
