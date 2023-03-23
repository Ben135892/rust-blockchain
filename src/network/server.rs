use super::local_transport::{self, new_local_transport, LocalTransport};
use crate::core::blockchain::{new_blockchain, Blockchain};

struct ServerOpts {
    listen_addr: String,
}
struct Server<'a> {
    pub opts: ServerOpts,
    pub local_transport: LocalTransport<'a>,
    pub chain: Blockchain,
}

// pub fn new_server(opts: ServerOpts) -> Server {
//     Server {
//         opts,
//         local_transport: new_local_transport(opts.listen_addr),
//         chain: new_blockchain(genesis_block())
//     }
// }

// fn genesis_block() -> Block {

// }
