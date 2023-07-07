use async_std::{
    prelude::*,
    task,
    net::{TcpListener, ToSocketAddrs, TcpStream}, io::BufReader,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;


fn main() {
    let future = accept_loop("127.0.0.1:8080");
    task::block_on(future);

    
}

async fn accept_loop(addr: impl ToSocketAddrs) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    let mut incoming = listener.incoming();


    while let Some(stream) = incoming.next().await{
        let stream = stream?;
        println!("Recieved from: {}", stream.peer_addr()? );
        let _handler = spawn_and_log_error(connection_loop(stream));
    }
    Ok(())
}

async fn connection_loop(stream: TcpStream) -> Result<()> {
    let reader = BufReader::new(stream);
    let mut lines = reader.lines();
    let login_line = match lines.next().await {
        None => Err("Peer Disconnected Immediately")?,
        Some(line) => line?,
    };
    println!("name: {}", login_line);
    while let Some(line) = lines.next().await {
        let line = line?;
        let (destination, message) = match line.find(":") {
            None => continue,
            Some(i) => (&line[..i], line[i + 1 ..].trim()),
        };
        let destination: Vec<String> = destination.split(',').map(|name| 
            name.trim().to_string()).collect();
        let message: String = message.to_string();
        
    }
    Ok(())
}

fn spawn_and_log_error<F>(fut: F) -> task::JoinHandle<()>
where F: Future<Output = Result<()>> + Send + 'static {
    task::spawn(async move {
            if let Err(e) = fut.await {
                eprintln!("{}", e)
            }
        }
    )
}
   
 
