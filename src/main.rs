use futures_util::SinkExt;
use tokio_tungstenite::accept_async;
use futures_util::stream::StreamExt;
use tokio::net::TcpListener;

mod model;
mod schema;
mod util;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:9001")
        .await.expect("Falha ao iniciar servidor");

    println!("Servidor WebSocket rodando em ws://127.0.0.1:9001");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.expect("Erro ao aceitar conexão");
            println!("Cliente conectado!");

            let (mut write, mut read) = ws_stream.split();

            while let Some(Ok(msg)) = read.next().await {
                println!("Recebido: {:?}", msg);
                write.send(msg).await.expect("Erro ao enviar resposta");
            }
        });
    }
}
