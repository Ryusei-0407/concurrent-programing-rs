use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::io;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let host = TcpListener::bind("127.0.0.1:10000").await.unwrap();

    loop {
        // Accept connection
        let (mut socket, addr) = host.accept().await?;
        println!("accept: {addr}");

        // Spawn
        tokio::spawn(async move {
            // Create buffer read/write object
            let (r, w) = socket.split(); // Split Read socket / Write socket
            let mut reader = io::BufReader::new(r);
            let mut writer = io::BufWriter::new(w);

            let mut line = String::new();
            loop {
                line.clear(); // Clear
                // async read line
                match reader.read_line(&mut line).await {
                    Ok(0) => {
                        println!("closed: {addr}");
                        return;
                    }
                    Ok(_) => {
                        print!("read: {addr}, {line}");
                        writer.write_all(line.as_bytes()).await.unwrap();
                        writer.flush().await.unwrap();
                    }
                    Err(e) => {
                        println!("error: {addr}, {e}");
                        return;
                    }
                }
            }
        });
    }
}
