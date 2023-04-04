use std::collections::HashMap;
use std::io::Write;
use std::net::{SocketAddr, TcpStream};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex, RwLock, RwLockWriteGuard};
use std::thread;
use std::time::Duration;

use super::rpc::{default_rpc_decode, RPCDecodeFunc, RPC};
use super::tcp_transport::TCPTransport;
use super::txpool::{self, TxPool};
use crate::core::block::{new_block, new_block_from_prev_header, new_header, Block, Header};
use crate::core::blockchain::Blockchain;
use crate::crypto::keypair::KeyPair;
use crate::network::rpc::{Decoded, Message, MESSAGE_TYPE_BLOCK};
use crate::network::tcp_transport::TcpPeer;

pub struct ServerOpts {
    pub listen_addr: String,
    pub seed_nodes: Vec<String>,
    pub key_pair: Option<KeyPair>,
    pub block_time: u32,
    pub rpc_decode_func: Option<RPCDecodeFunc>,
}
pub struct Server {
    pub opts: ServerOpts,

    pub tcp_transport: TCPTransport,
    pub peer_map: Arc<RwLock<HashMap<SocketAddr, TcpPeer>>>,
    pub peer_sender: Sender<TcpPeer>,
    pub peer_receiver: Receiver<TcpPeer>,

    pub rpc_decode_func: RPCDecodeFunc,
    pub rpc_sender: Sender<RPC>,
    pub rpc_receiver: Receiver<RPC>,

    pub chain: Arc<RwLock<Blockchain>>,
    pub mempool: Arc<RwLock<TxPool>>,

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
            tcp_transport: TCPTransport::new(opts.listen_addr.clone(), peer_sender.clone()),
            peer_map: Arc::new(RwLock::new(HashMap::new())),
            peer_sender: peer_sender,
            peer_receiver: peer_receiver,

            rpc_sender: rpc_sender,
            rpc_receiver: rpc_receiver,

            chain: Arc::new(RwLock::new(Blockchain::new(genesis_block()))),
            mempool: Arc::new(RwLock::new(TxPool::new())),

            is_validator: opts.key_pair.is_some(),
            rpc_decode_func: opts.rpc_decode_func.unwrap_or(default_rpc_decode),
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
        if self.is_validator {
            self.validator_loop();
        }
        loop {
            if let Ok(_) = self.quit_receiver.try_recv() {
                break;
            }
            if let Ok(tcp_peer) = self.peer_receiver.try_recv() {
                let addr = tcp_peer.stream.peer_addr().unwrap();
                println!("received peer {}", addr);
                let rpc_sender = self.rpc_sender.clone();
                let mut tcp_peer_clone = tcp_peer.clone();
                thread::spawn(move || tcp_peer_clone.read_loop(rpc_sender));

                self.peer_map.write().unwrap().insert(addr, tcp_peer);
            }
            if let Ok(rpc) = self.rpc_receiver.try_recv() {
                println!("{:?}", rpc);
                let decoded_message = (self.rpc_decode_func)(rpc);

                match decoded_message {
                    Ok(message) => match message.data {
                        Decoded::Block(block) => {
                            println!("{:?}", block);
                        }
                        Decoded::Transaction(transaction) => {
                            println!("{:?}", transaction);
                        }
                    },
                    Err(err) => {
                        println!("{}", err);
                    }
                }
                // let mut b_decode: Block = serde_json::from_slice(&rpc).unwrap();
                // println!("{:?}", b_decode);
            }
        }
    }

    fn bootstrap_network(&self) {
        for addr in &self.opts.seed_nodes {
            let addr = addr.clone();
            let peer_sender = self.peer_sender.clone();
            thread::spawn(move || {
                let addr = String::from("localhost") + &addr;
                let stream = TcpStream::connect(&addr).unwrap();
                let tcp_peer = TcpPeer::new(stream, true);

                peer_sender.send(tcp_peer).unwrap();
                thread::sleep(Duration::from_millis(500));
            });
        }
    }

    fn validator_loop(&mut self) {
        println!(
            "Starting validator loop with block time {}",
            self.opts.block_time
        );
        let blockchain = self.chain.clone();
        let peer_map = self.peer_map.clone();
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(3));
            let chain = blockchain.write().unwrap();
            let _ = create_new_block(chain);

            // broadcast block to peers

            let mut chain = blockchain.write().unwrap();
            let height = chain.height();
            let block_added = chain.get_block(height).unwrap();

            let peer_map = peer_map.read().unwrap();
            // peer_map.clone() clones every tcp peer every time we broadcast - very inefficient
            for (_addr, mut tcp_stream) in peer_map.clone().into_iter() {
                let message = Message::new(MESSAGE_TYPE_BLOCK, block_added.encode().into_bytes());
                tcp_stream.send(message.bytes());
            }
        });
    }
}

fn create_new_block(mut chain: RwLockWriteGuard<Blockchain>) {
    let height = chain.height();
    let h = chain.get_header(height).unwrap();
    let block = new_block_from_prev_header(h, vec![]);
    chain.add_block(block).unwrap();
    println!("adding block");
}

fn genesis_block() -> Block {
    let header = new_header(1, "".to_string(), "".to_string(), 0, 0);
    new_block(header, vec![])
}
