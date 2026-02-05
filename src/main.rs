use websocket::futures::Future;
use websocket::{ClientBuilder};
use websocket::client::Url;
use websocket::r#async::client::{Client, ClientNew};
fn main() {
    let ws_url = &Url::parse("wss://ws.backpack.exchange").unwrap();
    let mut connection = ClientBuilder::from_url(ws_url);
    let client = connection.connect_insecure().unwrap().into_stream();
    let _values = client.0.bytes().map(|item|{
        match item {
            Ok(item) => println!("{:#?}",item.to_string()),
            Err(_error) => panic!("Error")
        }
    });
    let mut async_client = connection.async_connect_insecure();

}
fn unbox<T>(value: Box<T>) -> T {
    
}