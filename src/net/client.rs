use std::sync::Arc;

use crate::protos::msg::*;
use crate::protos::msg_grpc::*;

use grpcio::{ChannelBuilder, EnvBuilder};

pub struct SideBoatClient {
    status: i32,
    client: TalkerClient,
}

impl SideBoatClient {
    pub fn new(ip: &str) -> SideBoatClient {
        let env = Arc::new(EnvBuilder::new().build());
        let ch = ChannelBuilder::new(env).connect(ip);
        let client = TalkerClient::new(ch);
        SideBoatClient {
            status: 1,
            client: client,
        }
    }
    pub fn send_message(&self, msg: &str) -> InitSystem {
        let mut req = InitSystem::default();
        req.set_ip_address(msg.to_owned());
        let reply = self.client.talk(&req).expect("rpc");
        reply
    }
}
