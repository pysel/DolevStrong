use std::net::{TcpListener, TcpStream};
use std::io::Error;
use std::time::{Instant, Duration};

use crate::utils::vec::unwrap_streams;

use crate::node;

impl node::Node {
    // bind_and_wait_Config binds a listening port of this node and waits for other peers to connect to this port
    pub fn bind_and_wait_connection(&mut self) {
        let listener: TcpListener = TcpListener::bind(self.config.listen_socket.clone())
            .expect("Failed to bind");

        let num_peers: i32 = self.config.num_peers.clone();
        let mut peers: Vec<TcpStream> = vec![];

        println!("Listening on socket {}", self.config.listen_socket);
    
        loop { // wait until all peers are connected
            match listener.accept() {
                Ok((stream, _)) => {
                    peers.push(stream);
        
                    if peers.len() == num_peers.try_into().expect("Could not convert waiting_for_num_connections into i32") {
                        break;
                    }
                }
                Err(e) => {
                    println!("ERROR CONNECTING: {e}")
                }
            }
        }
        
        self.set_listen_stream(Some(peers));
        // TODO: consider adding timeout
    }

    // connect_to_peers tries connecting to peers, returns Result of all attempts
    fn connect_to_peers(&self) -> Vec<Result<TcpStream, Error>> {
        let mut streams: Vec<Result<TcpStream, Error>> = Vec::new();
        for peer in self.config.peers.clone() {
            let stream = TcpStream::connect(peer.socket.clone());
            streams.push(stream);
        }
        streams
    }

    fn connect_until_success(&mut self) -> Option<Error> {
        let start = Instant::now();

        loop {
            let streams = self.connect_to_peers();
            if let Ok(streams) = unwrap_streams(streams) {
                self.config.set_write_streams(Some(streams));
                return None;
            }

            if start.elapsed() > Duration::from_secs(10) {
                break Some(Error::new(std::io::ErrorKind::NotConnected, "Timeout triggered before self could connect to all peers"));
            }
        }
    }


    fn set_listen_stream(&mut self, peers: Option<Vec<TcpStream>>) {
        match &peers {
            Some(p) => {
                if self.config.num_peers != p.len().try_into().expect("Could not convert peers' length to i32") {
                    panic!("Not all peers connected to node at port {}", self.config.listen_socket)
                }
                self.config.set_listen_streams(peers);
            }
            None => {
                panic!("Attempt to set empty peers")
            }
        }
    }
}
