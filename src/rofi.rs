use std::{os::unix::net::{UnixListener, UnixStream}, io::{Read, BufRead}, io::{BufReader, BufWriter, Write}};
use crate::notification::Manager;

pub struct RofiServer {
    socket_path: String,
    db: Manager,
}

// See https://github.com/DaveDavenport/Rofication/blob/master/rofication-daemon.py#LL155C1-L170C87
pub enum RofiCommand {
    COUNT,
    LIST,
    DELETE_ONE,
    DELETE_SIMILAR,
    DELETE_APPS,
    MARK_SEEN,
/*
    # Get number of notifications
    if command == "num":
        self.communication_command_num(connection)
    # Getting a listing.
    elif command == "list":
        self.communication_command_send_list(connection)
    # Dismiss and item.
    elif command == "del":
        self.communication_command_delete(connection,data.split(':')[1])
    elif command == "dels":
        self.communication_command_delete_similar(connection,data.split(':')[1])
    elif command == "dela":
        self.communication_command_delete_apps(connection,data.split(':')[1])
    # Saw an item, this sets the urgency to normal.
    elif command == "saw":
        self.communication_command_saw(connection, data.split(':')[1])
         */
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
                    v => {
                        println!("unknown command: '{}'", v);
                        None
                    }
                }

            },
            None => None
        }
    }
}

impl  RofiServer {
    pub fn new(socket_path: String, db: Manager) -> RofiServer {
        return RofiServer { socket_path, db }
    }

    pub fn start(&self) -> std::io::Result<()> {
        println!("Binding to path {}", &self.socket_path);
        let listener = UnixListener::bind(&self.socket_path)?;
    
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    Self::handle_request(stream);                    
                }
                Err(err) => {
                    println!("Failed to initialize socket listener: {}", err);
                    break;
                }
            }
        }
        Ok(())
    }

    fn handle_request(stream: UnixStream) {        
        let mut client_in = BufReader::new(&stream);
        let mut client_out = BufWriter::new(&stream);

        let mut line = String::new();
        let _ = client_in.read_line(&mut line).expect("unable to read");

        let line = line.trim();
        println!("Message: '{}'", line);

        match RofiCommand::parse(&line) {
            Some(RofiCommand::COUNT) => {
                client_out.write("0".as_bytes()).unwrap();
                client_out.flush().unwrap();
            },
            _ => {
                println!("Unable to parse message, no action taken");
            }
        }

    }
}
