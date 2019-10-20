extern crate tokio;

use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:7543".parse().unwrap();
    let mut listener = TcpListener::bind(&addr).unwrap();
    loop{
        let (mut socket,_) = listener.accept().await.unwrap();
        tokio::spawn( async move {
            let mut buf = [0; 1024];
            loop {
                let n = match socket.read(&mut buf).await{
                    Ok(n) if n == 0 => return,
                    Ok(n) => {
                        println!("Read Bytes {}",n);
                        n
                    },
                    Err(e)=> {
                        println!("read error {:?}",e);
                        return;
                    }
                };

                let r = match socket.write_all(&buf[0..n]).await {
                    Ok(r) => {
                        println!("write Bytes {}",n);
                        r
                    },
                    Err(e) =>{
                        println!("write error {:?}",e);
                        return;
                    }
                };
            }
        });
    }
}
