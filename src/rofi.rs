use std::{os::unix::net::UnixListener, io::Read};

pub struct RofiServer {
    socket_path: String,
}

impl RofiServer {
    pub fn new(socket_path: String) -> RofiServer {
        return RofiServer { socket_path }
    }

    pub fn start(&self) -> std::io::Result<()> {
        println!("Binding to path {}", &self.socket_path);
        let listener = UnixListener::bind(&self.socket_path)?;
    
        // accept connections and process them, spawning a new thread for each one
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut response = String::new();
                    stream.read_to_string(&mut response)?;
                    println!("Received on socket: {response}");
                }
                Err(err) => {
                    /* connection failed */
                    println!("Error: {}", err);
                    break;
                }
            }
        }
        Ok(())
    }
}
