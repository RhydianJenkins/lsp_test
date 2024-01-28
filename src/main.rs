use serde_json::Value;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    println!("Server running on {}", addr);

    loop {
        let (mut socket, _) = listener
            .accept()
            .await
            .expect("Failed to accept connection");

        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            loop {
                match socket.read(&mut buffer).await {
                    Ok(size) => {
                        if size == 0 {
                            break;
                        }
                        let request = String::from_utf8_lossy(&buffer[..size]);
                        if request.contains("textDocument/codeAction") {
                            println!("Received codeAction request");

                            let response = serde_json::to_string(&Value::String(
                                "Handled codeAction".to_owned(),
                            ))
                            .expect("Failed to serialize response");

                            if let Err(e) = socket.write_all(response.as_bytes()).await {
                                eprintln!("Failed to send response: {}", e);
                            }

                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to read from socket: {}", e);
                        break;
                    }
                }
            }
        });
    }
}
