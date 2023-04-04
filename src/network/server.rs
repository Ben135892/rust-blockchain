use std::collections::HashMap;
use std::io::Write;
use std::net::{SocketAddr, TcpStream};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use super::tcp_transport::TCPTransport;
use super::txpool::{self, TxPool};
use crate::core::block::{new_block, new_header, Block, Header};
use crate::core::blockchain::Blockchain;

pub struct ServerOpts {
    pub listen_addr: String,
}
pub struct Server {
    pub opts: ServerOpts,

    pub tcp_transport: TCPTransport,
    pub peer_map: HashMap<SocketAddr, Arc<TcpStream>>,
    pub peer_sender: Sender<Arc<TcpStream>>,
    pub peer_receiver: Receiver<Arc<TcpStream>>,

    pub rpc_sender: Sender<Vec<u8>>,
    pub rpc_receiver: Receiver<Vec<u8>>,

    pub chain: Blockchain,
    pub mempool: TxPool,

    pub quit_sender: Sender<()>,
    pub quit_receiver: Receiver<()>,
}

impl Server {
    pub fn new(opts: ServerOpts) -> Self {
        let (quit_sender, quit_receiver) = channel();
        let (peer_sender, peer_receiver) = channel();
        let (rpc_sender, rpc_receiver) = channel();

        Server {
            tcp_transport: TCPTransport::new(
                opts.listen_addr.clone(),
                peer_sender.clone(),
                rpc_sender.clone(),
            ),
            peer_map: HashMap::new(),
            peer_sender: peer_sender,
            peer_receiver: peer_receiver,

            rpc_sender: rpc_sender,
            rpc_receiver: rpc_receiver,

            chain: Blockchain::new(genesis_block()),
            opts,
            mempool: TxPool::new(),

            quit_sender,
            quit_receiver,
        }
    }

    pub fn start(&mut self) {
        let tr = &self.tcp_transport;
        tr.start();
        thread::sleep(Duration::from_secs(1));
        loop {
            if let Ok(_) = self.quit_receiver.try_recv() {
                break;
            }
            if let Ok(tcp_stream) = self.peer_receiver.try_recv() {
                let addr = tcp_stream.as_ref().peer_addr().unwrap();
                println!("received peer {}", addr);

                // tcp_stream
                //     .as_ref()
                //     .write_all(b"hello world!dasdadsadada")
                //     .unwrap();

                self.peer_map.insert(addr, tcp_stream);
            }
            if let Ok(rpc) = self.rpc_receiver.try_recv() {
                println!("{:?}", rpc);
            }
        }
    }
}

fn genesis_block() -> Block {
    let header = new_header(1, "".to_string(), "".to_string(), 0, 0);
    new_block(header, vec![])
}
