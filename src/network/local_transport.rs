use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct LocalTransport<'a> {
    pub addr: String,
    rpc_sender: Sender<Vec<u8>>,
    rpc_receiver: Receiver<Vec<u8>>,
    peers: HashMap<String, &'a LocalTransport<'a>>,
}

pub fn new_local_transport<'a>(addr: String) -> LocalTransport<'a> {
    let (sender, receiver) = channel();
    LocalTransport {
        addr,
        rpc_sender: sender,
        rpc_receiver: receiver,
        peers: HashMap::new(),
    }
}

impl<'a> LocalTransport<'a> {
    pub fn consume(&self) -> &Receiver<Vec<u8>> {
        &self.rpc_receiver
    }

    pub fn connect(&mut self, tr: &'a LocalTransport<'a>) {
        self.peers.insert(tr.addr.clone(), tr);
    }

    pub fn send_message(&self, to: String, payload: Vec<u8>) -> Result<(), String> {
        match self.peers.get(&to) {
            Some(peer) => match peer.rpc_sender.send(payload) {
                Ok(()) => Ok(()),
                Err(_) => Err("Failed to send message to peer channel".to_string()),
            },
            None => Err(format!("Could not send message to unknown peer {}", to)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut tr_local = new_local_transport("local".to_string());
        let tr_remote = new_local_transport("remote".to_string());

        tr_local.connect(&tr_remote);
        tr_local
            .send_message(tr_remote.addr.clone(), vec![0, 1, 2])
            .unwrap();
        tr_local
            .send_message(tr_remote.addr.clone(), vec![3, 4, 5])
            .unwrap();

        let c = tr_remote.consume();
        println!("{:?}", c.recv().unwrap());
        println!("{:?}", c.recv().unwrap());
        println!("{:?}", c.recv().unwrap());
        println!("end");
    }
}
