use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6942").await.unwrap();
    let (sender, _receiver) = broadcast::channel(10);

    loop {
        let (mut socket, address) = listener.accept().await.unwrap();

        let local_sender = sender.clone();
        let mut receiver = local_sender.subscribe();

        tokio::spawn(async move {
            let (socket_reader, mut socket_writer) = socket.split();
            let mut reader = BufReader::new(socket_reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                   result = reader.read_line(&mut line) => {
                       if result.unwrap() == 0 {
                           break;
                       }
                        local_sender.send((line.clone(), address)).unwrap();
                        line.clear();
                   },
                    result = receiver.recv() => {
                        let (msg, other_address) = result.unwrap();

                        if address != other_address {

                        socket_writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}
