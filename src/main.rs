use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use crate::network::server::{Server, ServerOpts};

mod core;
mod crypto;
mod network;
mod types;

fn main() {
    let mut local = Server::new(ServerOpts {
        listen_addr: "3000".to_string(),
    });

    let mut remote = Server::new(ServerOpts {
        listen_addr: "4000".to_string(),
    });

    thread::spawn(move || {
        local.start();
    });

    thread::sleep(Duration::from_secs(1));
    let mut stream = TcpStream::connect("localhost:3000").unwrap();

    println!("fiunished wr");
    loop {}
}
