use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{mpsc::Sender, Arc, Mutex},
    thread,
};

pub struct TcpPeer {
    pub stream: TcpStream,
    pub outgoing: bool,
}

impl TcpPeer {
    pub fn new(stream: TcpStream, outgoing: bool) -> Self {
        Self {
            stream: stream,
            outgoing: outgoing,
        }
    }

    pub fn send(&mut self, data: Vec<u8>) {
        self.stream.write_all(&data).unwrap();
    }

    pub fn read_loop(&mut self, rpc_sender: Sender<Vec<u8>>) {
        loop {
            let mut buffer = [0u8; 2048];
            let n = self.stream.read(&mut buffer).unwrap();
            println!("received {} bytes", n);
            let b = buffer[0..n].to_vec();
            rpc_sender.send(b).unwrap();
        }
    }
}

pub struct TCPTransport {
    pub listen_addr: String,
    pub peer_sender: Sender<TcpPeer>,
    pub rpc_sender: Sender<Vec<u8>>,
}

impl TCPTransport {
    pub fn new(
        listen_addr: String,
        peer_sender: Sender<TcpPeer>,
        rpc_sender: Sender<Vec<u8>>,
    ) -> Self {
        TCPTransport {
            listen_addr: listen_addr,
            peer_sender: peer_sender,
            rpc_sender: rpc_sender,
        }
    }

    pub fn start(&self) {
        let listener = TcpListener::bind("localhost:".to_string() + &self.listen_addr).unwrap();
        let peer_sender = self.peer_sender.clone();
        let rpc_sender = self.rpc_sender.clone();

        thread::spawn(move || {
            loop {
                // listen for new incoming connections
                let (socket, _addr) = listener.accept().unwrap();
                let tcp_peer = TcpPeer::new(socket, false);
                println!("new connection from {:?}", _addr);

                peer_sender.send(tcp_peer).unwrap();
            }
        });
    }

    // fn read_loop(&self, socket: Arc<TcpStream>) {
    //     loop {
    //         let mut buffer = [0u8; 2048];
    //         let n = socket.as_ref().read(&mut buffer).unwrap();
    //         println!("received {} bytes", n);
    //         //self.rpc
    //     }
    // }
}
