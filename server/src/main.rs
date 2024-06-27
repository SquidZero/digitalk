use libdigitalk::{Channel, Message};
use std::future::IntoFuture;
use std::io::Read;
use std::pin::pin;
use std::process::exit;
use std::str;
use std::sync::{Arc, Mutex};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9297").await?;

    //    let (mut rd, mut wr) = socket.split();
    loop {
        let (mut socket, _) = listener.accept().await?;
        let input = Arc::new(Mutex::new(vec![]));
        let input_ptr = Arc::clone(&input);
        let input_binding = input_ptr.lock().unwrap();
        let input_str = match str::from_utf8(input_binding.as_slice()) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            match socket.read(&mut buf).await {
                Ok(0) => {
                    println!("return 0");
                    return;
                }
                Ok(n) => {
                    if buf[..n] == *"exit\n".as_bytes() {
                        exit(0);
                    }
                    println!("{:?}", &buf[..n]);
                    {
                        let mut i = input.lock().unwrap();
                        {
                            *i = buf[..n].to_vec();
                        }
                    }

                    if socket.write_all(&buf[..n]).await.is_err() {
                        println!("WRITE ERROR");
                        return;
                    }
                }
                Err(_) => {
                    println!("ERROR");
                    return;
                }
            }
        });
    }
}
