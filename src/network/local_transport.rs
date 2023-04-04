// use std::collections::HashMap;
// use std::sync::RwLock;
// use tokio::sync::mpsc::{channel, Receiver, Sender};

// pub struct LocalTransport<'a> {
//     pub addr: String,
//     lock: RwLock<()>,
//     pub rpc_sender: Sender<Vec<u8>>,
//     rpc_receiver: Receiver<Vec<u8>>,
//     peers: HashMap<String, &'a LocalTransport<'a>>,
// }

// impl<'a> LocalTransport<'a> {
//     pub fn new(addr: String) -> Self {
//         let (rpc_sender, rpc_receiver) = channel(1);
//         LocalTransport {
//             addr,
//             lock: RwLock::new(()),
//             rpc_sender: rpc_sender,
//             rpc_receiver: rpc_receiver,
//             peers: HashMap::new(),
//         }
//     }

//     pub fn consume(&mut self) -> &mut Receiver<Vec<u8>> {
//         &mut self.rpc_receiver
//     }

//     pub fn connect(&mut self, tr: &'a LocalTransport<'a>) {
//         let _unused = self.lock.write().unwrap();
//         self.peers.insert(tr.addr.clone(), tr);
//     }

//     pub async fn send_message(&self, to: String, payload: Vec<u8>) -> Result<(), String> {
//         let _unused = self.lock.read().unwrap();
//         match self.peers.get(&to) {
//             Some(peer) => match peer.rpc_sender.send(payload).await {
//                 Ok(()) => Ok(()),
//                 Err(_) => Err("Failed to send message to peer channel".to_string()),
//             },
//             None => Err(format!("Could not send message to unknown peer {}", to)),
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use std::sync::mpsc::channel;

//     use super::*;

//     #[tokio::test]
//     async fn test() {
//         let mut tr_local = LocalTransport::new("local".to_string());
//         let mut tr_remote = LocalTransport::new("remote".to_string());

//         tr_local.connect(&tr_remote);
//         tr_local
//             .send_message(tr_remote.addr.clone(), vec![0, 1, 2])
//             .await
//             .unwrap();

//         // tr_local
//         //     .send_message(tr_remote.addr.clone(), vec![3, 4, 5])
//         //     .await
//         //     .unwrap();

//         let c = tr_remote.consume();
//         println!("{:?}", c.recv().await.unwrap());

//         println!("end");
//     }
// }
