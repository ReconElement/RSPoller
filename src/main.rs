use std::io::Read;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use websocket::futures::Future;
use websocket::{ClientBuilder};
use websocket::client::Url;
use websocket::r#async::client::{Client, ClientNew};
// fn main() {
//     let ws_url = &Url::parse("wss://ws.backpack.exchange").unwrap();
//     let mut connection = ClientBuilder::from_url(ws_url);
//     let client = connection.connect_insecure().unwrap().into_stream();
//     let _values = client.0.bytes().map(|item|{
//         match item {
//             Ok(item) => println!("{:#?}",item.to_string()),
//             Err(_error) => panic!("Error")
//         }
//     });
//     let mut async_client = connection.async_connect_insecure();

// }

#[tokio::main]
async fn main(){
    let ws_url = &Url::parse("wss://ws.backpack.exchange").unwrap();
    let mut connection = ClientBuilder::from_url(ws_url);
    //asynchronous connection 
    let mut async_client = connection.async_connect_insecure();
    let mut stream = async_client.deref_mut().wait();
    let mut stream = stream.unwrap().0.into_inner();
    //read 
    let mut buf = [0;10];
    let value = stream.read(&mut buf).unwrap();
    println!("{:#?}",value);
}
