mod net;
mod protos;

use net::client::SideBoatClient;

fn main() {
    let x = 1;
    let client = SideBoatClient::new("ipv4:127.0.0.1:50051");
    for i in 0..2 {
        let mut msg = String::from("Message: ");
        msg.push_str(i.to_string().as_ref());
        println!("{}", client.send_message(msg.as_ref()).get_ip_address());
    }
    loop {}
}
