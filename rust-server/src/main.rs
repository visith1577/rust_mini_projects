use tokio::{net::TcpStream, io::AsyncWriteExt};
use std::error::Error;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:8080";
    let client = TcpStream::connect(addr);

    if let Ok(mut stream) = client.await {
        println!("Stream is created");
        let str = stream.write_all(b"Hello world \n").await;
        println!("wrote to stream: success = {:?} ", str.is_ok());
    }

    Ok(())
}
