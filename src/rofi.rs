use std::{os::unix::net::{UnixListener, UnixStream}, io::BufRead, io::{BufReader, BufWriter, Write}};
use crate::notification::NotificationStore;

/// Provides a service for roficiation clients. See https://github.com/DaveDavenport/Rofication
pub struct RofiServer {
    socket_path: String,
    db: NotificationStore,
}

// See https://github.com/DaveDavenport/Rofication/blob/master/rofication-daemon.py#LL155C1-L170C87
pub enum RofiCommand {
    COUNT,
    LIST,
    DELETE_ONE,
    DELETE_SIMILAR,
    DELETE_APPS,
    MARK_SEEN,
}

impl RofiCommand {
    fn parse(client_request: &str) -> Option<RofiCommand> {
        let mut token_iter = client_request.split(" ").into_iter();

        match token_iter.next() {
            Some(command) => {
                match command {
                    "num" => Some(Self::COUNT),
                    "list" => Some(Self::LIST),
                    "del" => Some(Self::DELETE_ONE),
                    "dels" => Some(Self::DELETE_SIMILAR),
                    "dela" => Some(Self::DELETE_APPS),
                    "saw" => Some(Self::MARK_SEEN),
                    other => {
                        println!("unknown command: '{}'", other);
                        None
                    }
                }

            },
            None => None
        }
    }
}

impl  RofiServer {
    pub fn new(socket_path: String, db: NotificationStore) -> RofiServer {
        return RofiServer { socket_path, db }
    }

    pub fn start(&self) -> std::io::Result<()> {
        println!("Binding to path {}", &self.socket_path);
        let listener = UnixListener::bind(&self.socket_path)?;
    
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    self.handle_request(stream);                    
                }
                Err(err) => {
                    println!("Failed to initialize socket listener: {}", err);
                    break;
                }
            }
        }
        Ok(())
    }

    fn handle_request(&self, stream: UnixStream) {        
        let mut client_in = BufReader::new(&stream);
        let mut client_out = BufWriter::new(&stream);

        let mut line = String::new();
        let _ = client_in.read_line(&mut line).expect("unable to read");

        let line = line.trim();
        println!("Rofication client request: '{}'", line);

        match RofiCommand::parse(&line) {
            Some(RofiCommand::COUNT) => {                
                client_out.write(self.db.count().to_string().as_bytes()).unwrap();
                client_out.flush().expect("Sending response back to client")
            },
            Some(RofiCommand::LIST) => {
                let elems = self.db.items();
                let response = serde_json::to_string(&elems).unwrap();
                client_out.write(&response.as_bytes()).unwrap();
                client_out.flush().expect("Sending response back to client")
            },
            _ => {
                println!("Unable to parse message, no action taken: {}", &line);
            }
        }

    }
}
