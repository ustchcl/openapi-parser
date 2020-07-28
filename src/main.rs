use openapi::{from_reader_json};
use hyper::{body::HttpBody as _, Client};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
pub async fn main() -> Result<()> {
    println!("{}", "hello rust");
    
    let client = Client::new();
    let uri = "http://120.25.124.141:8999/lwtraining/debug/api/openapi.json".parse::<hyper::Uri>()?;
    let mut resp = client.get(uri).await?;
    println!("Response: {}", resp.status());
    println!("Headers: {:#?}\n", resp.headers());
    let buf = hyper::body::to_bytes(resp).await?;
    let read = std::io::Cursor::new(buf);
    if let Ok(api) = from_reader_json(read) {
        println!("{:?}", api);
    }
    Ok(())


}
