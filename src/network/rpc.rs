use crate::core::{block::Block, transaction::Transaction};
use std::{fmt::Error, net::SocketAddr};

pub type RPCDecodeFunc = fn(rpc: RPC) -> Result<DecodedMessage, String>;

type MessageType = u8;

pub const MESSAGE_TYPE_TX: MessageType = 0x1;
pub const MESSAGE_TYPE_BLOCK: MessageType = 0x2;
// const MessageTypeGetBlocks: MessageType = 0x3;
// const MessageTypeStatus: MessageType = 0x4;
// const MessageTypeGetStatus: MessageType = 0x5;
// const MessageTypeBlocks: MessageType = 0x6;

#[derive(Debug)]
pub struct RPC {
    pub from: SocketAddr,
    pub data: Vec<u8>,
}

pub enum Decoded {
    Transaction(Transaction),
    Block(Block),
}

// use Message to format message to send bytes
pub struct Message {
    header: u8,
    data: Vec<u8>,
}

impl Message {
    pub fn new(header: u8, data: Vec<u8>) -> Self {
        let mut message = Message { header, data };
        message.format();
        message
    }

    pub fn format(&mut self) {
        self.data.insert(0, self.header);
    }

    pub fn bytes(self) -> Vec<u8> {
        return self.data;
    }
}

pub struct DecodedMessage {
    pub from: SocketAddr,
    pub data: Decoded,
}

pub fn default_rpc_decode(rpc: RPC) -> Result<DecodedMessage, String> {
    if rpc.data.len() < 1 {
        return Err(String::from("RPC data is empty"));
    }
    let message_type = *rpc.data.get(0).unwrap();
    match message_type {
        MESSAGE_TYPE_TX => {
            let tx_data = &rpc.data[1..];

            if let Ok(tx_decode) = serde_json::from_slice::<Transaction>(tx_data) {
                return Ok(DecodedMessage {
                    from: rpc.from,
                    data: Decoded::Transaction(tx_decode),
                });
            } else {
                return Err(String::from("could not parse transaction RPC"));
            }
        }
        MESSAGE_TYPE_BLOCK => {
            let block_data = &rpc.data[1..];

            if let Ok(block_decode) = serde_json::from_slice::<Block>(block_data) {
                return Ok(DecodedMessage {
                    from: rpc.from,
                    data: Decoded::Block(block_decode),
                });
            } else {
                return Err(String::from("could not parse block RPC"));
            }
        }
        _ => {
            return Err(format!("invalid message header {}", message_type));
        }
    }
}
