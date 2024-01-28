use serde::Serialize;
use serde_json::Value;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[derive(Serialize)]
struct CodeActionResponse {
    jsonrpc: &'static str,
    id: usize,
    result: ResultObject,
}

#[derive(Serialize)]
struct ResultObject {
    actions: Vec<Action>,
}

#[derive(Serialize)]
struct Action {
    title: &'static str,
    kind: &'static str,
    executeCommand: Command,
}

#[derive(Serialize)]
struct Command {
    title: &'static str,
    command: &'static str,
    arguments: Vec<&'static str>,
}

fn generate_code_action_response(request_id: usize) -> String {
    let response = CodeActionResponse {
        jsonrpc: "2.0",
        id: request_id,
        result: ResultObject {
            actions: vec![Action {
                title: "Print 'Hello, Neovim!'",
                kind: "quickfix",
                executeCommand: Command {
                    title: "Print",
                    command: "lua.execute",
                    arguments: vec!["print('Hello, Neovim!')"],
                },
            }],
        },
    };

    let json_string = serde_json::to_string(&response).unwrap();

    json_string
}

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
                                generate_code_action_response(1).to_owned(),
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
