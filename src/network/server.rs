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
use crate::crypto::keypair::KeyPair;
use crate::network::tcp_transport::TcpPeer;

pub struct ServerOpts {
    pub listen_addr: String,
    pub seed_nodes: Vec<String>,
    pub key_pair: Option<KeyPair>,
    pub block_time: u32,
}
pub struct Server {
    pub opts: ServerOpts,

    pub tcp_transport: TCPTransport,
    pub peer_map: HashMap<SocketAddr, TcpPeer>,
    pub peer_sender: Sender<TcpPeer>,
    pub peer_receiver: Receiver<TcpPeer>,

    pub rpc_sender: Sender<Vec<u8>>,
    pub rpc_receiver: Receiver<Vec<u8>>,

    pub chain: Blockchain,
    pub mempool: TxPool,

    pub is_validator: bool,

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
            mempool: TxPool::new(),

            is_validator: opts.key_pair.is_some(),
            opts,

            quit_sender,
            quit_receiver,
        }
    }

    pub fn start(&mut self) {
        let tr = &self.tcp_transport;
        tr.start();
        thread::sleep(Duration::from_secs(1));
        self.bootstrap_network();
        loop {
            if let Ok(_) = self.quit_receiver.try_recv() {
                break;
            }
            if let Ok(tcp_peer) = self.peer_receiver.try_recv() {
                let addr = tcp_peer.stream.peer_addr().unwrap();
                println!("received peer {}", addr);
                let rpc_sender = self.rpc_sender.clone();

                let mut tcp_peer_clone = TcpPeer {
                    stream: tcp_peer.stream.try_clone().unwrap(),
                    outgoing: tcp_peer.outgoing,
                };
                thread::spawn(move || tcp_peer_clone.read_loop(rpc_sender));

                // tcp_stream
                //     .as_ref()
                //     .write_all(b"hello world!dasdadsadada")
                //     .unwrap();
                self.peer_map.insert(addr, tcp_peer);
            }
            if let Ok(rpc) = self.rpc_receiver.try_recv() {
                println!("{:?}", rpc);
            }
        }
    }

    fn bootstrap_network(&self) {
        for addr in &self.opts.seed_nodes {
            let addr = addr.clone();
            let peer_sender = self.peer_sender.clone();
            thread::spawn(move || {
                let addr = String::from("localhost") + &addr;
                let mut stream = TcpStream::connect(&addr).unwrap();
                //stream.write_all(b"hey").unwrap();
                let tcp_peer = TcpPeer::new(stream, true);

                peer_sender.send(tcp_peer).unwrap();
                thread::sleep(Duration::from_millis(500));
            });
        }
    }

    pub fn validator_loop(&self) {
        println!(
            "Starting validator loop with block time {}",
            self.opts.block_time
        );
    }
}

fn genesis_block() -> Block {
    let header = new_header(1, "".to_string(), "".to_string(), 0, 0);
    new_block(header, vec![])
}
