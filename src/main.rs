use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use crate::{
    crypto::keypair::KeyPair,
    network::server::{Server, ServerOpts},
};

mod core;
mod crypto;
mod network;
mod types;

fn main() {
    let mut local = Server::new(ServerOpts {
        listen_addr: "3000".to_string(),
        key_pair: Some(KeyPair::new(0)),
        block_time: 12,
        seed_nodes: vec![String::from(":4000")],
    });

    let mut remote = Server::new(ServerOpts {
        listen_addr: "4000".to_string(),
        key_pair: None,
        block_time: 12,
        seed_nodes: vec![],
    });

    thread::spawn(move || {
        local.start();
    });

    thread::spawn(move || {
        remote.start();
    });

    thread::sleep(Duration::from_secs(1));
    let mut stream = TcpStream::connect("localhost:3000").unwrap();

    println!("fiunished wr");
    loop {}
}
