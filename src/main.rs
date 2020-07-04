mod api_client;
mod mail;
mod sns;

use crate::sns::sns_client::*;
use crate::api_client::api_client::*;

fn main() {
    let factory = SnsClientFactory::new("reta");
    let client = factory.build();
    let message_result = client.post("hello!");
    println!("{}", message_result);
    let bye = client.post("bye!");
    println!("{}", bye);
}
