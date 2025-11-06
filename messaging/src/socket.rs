//! This module is only concerned with the communication protocol primitives
//! meaning read/write - it only makes sure the correct number of bytes is being read
//! Any protobuf operations (serialization/deserialization) should be handled by consumers
use std::io::{Error, ErrorKind, Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;

const SOCKET_PATH: &str = "/tmp/ajazz-launchpad-socket";

pub struct Client {
    unix_stream: UnixStream,
}

pub struct Server {
    unix_listener: UnixListener,
    unix_streams: Vec<UnixStream>,
}

pub trait MessageSender {
    fn send_message(&mut self, msg: &[u8]) -> Result<(), Error>;
}

pub trait MessageReceiver {
    fn read_message(&mut self) -> Result<Vec<u8>, Error>;
}

/// A blocking (by default) consumer connection to the server.
impl Client {
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            unix_stream: UnixStream::connect(SOCKET_PATH)?,
        })
    }
}

impl MessageSender for Client {
    /// A blocking call to write to the server
    ///
    /// * `msg` - the byte array containing the encoded message
    fn send_message(&mut self, msg: &[u8]) -> Result<(), Error> {
        let len = msg.len() as u64;

        // Write length header first
        self.unix_stream.write_all(&len.to_le_bytes())?;
        // Write message content
        self.unix_stream.write_all(msg)?;

        Ok(())
    }
}

impl MessageReceiver for Client {
    /// Blocking read of any messages broadcasted from the client.
    /// **Warning**: The output format is a Vector of bytes to reduce
    /// the complexity of returning an owned value, handle that appropriately.
    fn read_message(&mut self) -> Result<Vec<u8>, Error> {
        let mut length_buf = [0u8; 8];
        self.unix_stream.read_exact(&mut length_buf)?;
        let len = u64::from_le_bytes(length_buf);

        validate_message_length(len)?;

        let mut message_buf = vec![0u8; len as usize];
        self.unix_stream.read_exact(&mut message_buf)?;

        Ok(message_buf)
    }
}

impl Server {
    pub fn new() -> Result<Self, Error> {
        if Path::new(SOCKET_PATH).exists() {
            std::fs::remove_file(SOCKET_PATH)?;
        }

        let unix_listener = UnixListener::bind(SOCKET_PATH)?;

        Ok(Self {
            unix_listener,
            unix_streams: vec![],
        })
    }

    /// _Blocking_ call to accept an incoming connection. Will add the connection
    /// to the list of connections if it is added successfully, otherwise the resulting
    /// error will need to be handled accordingly
    pub fn accept_connection(&mut self) -> Result<(), Error> {
        match self.unix_listener.accept() {
            Ok((stream, _addr)) => {
                stream.set_nonblocking(true)?;
                self.unix_streams.push(stream);
            }
            Err(e) => return Err(e),
        }

        Ok(())
    }

    /// Removes any invalid connections from the list of `unix_streams`
    pub fn cleanup_disconnected(&mut self) {
        self.unix_streams
            .retain(|stream| stream.peer_addr().is_ok());
    }

    pub fn connected_clients(&self) -> usize {
        self.unix_streams.len()
    }
}

impl MessageSender for Server {
    /// Blocking message write, sends a byte-encoded message to each of the clients
    /// The transformation of the message should be handled by the called
    ///
    /// * `msg` - the byte array containing the message to send to all clients
    fn send_message(&mut self, msg: &[u8]) -> Result<(), Error> {
        let msg_bytes = msg;
        let len = msg_bytes.len() as u64;
        let mut failed_streams = Vec::new();

        for (index, stream) in self.unix_streams.iter_mut().enumerate() {
            let write_result = stream
                .write_all(&len.to_le_bytes())
                .and_then(|_| stream.write_all(msg_bytes));

            if write_result.is_err() {
                failed_streams.push(index);
            }
        }

        for &index in failed_streams.iter().rev() {
            self.unix_streams.remove(index);
        }

        Ok(())
    }
}

impl MessageReceiver for Server {
    /// Non-blocking read for all connected clients. Will pass if there is
    /// currently nothing to read, otherwise it will return the first received message
    fn read_message(&mut self) -> Result<Vec<u8>, Error> {
        for stream in &mut self.unix_streams {
            match try_read_message(stream) {
                Ok(Some(msg)) => return Ok(msg),
                Ok(None) => continue, // No data available on this stream
                Err(_) => continue,   // Error on this stream, try next
            }
        }

        Err(Error::new(ErrorKind::WouldBlock, "No messages available"))
    }
}

fn try_read_message(stream: &mut UnixStream) -> Result<Option<Vec<u8>>, Error> {
    let mut length_buf = [0u8; 8];
    match stream.read_exact(&mut length_buf) {
        Ok(_) => {}
        Err(ref e) if e.kind() == ErrorKind::WouldBlock => return Ok(None),
        Err(e) => return Err(e),
    }

    let len = u64::from_le_bytes(length_buf);

    validate_message_length(len)?;

    let mut message_buf = vec![0u8; len as usize];
    stream.read_exact(&mut message_buf)?;

    Ok(Some(message_buf))
}

fn validate_message_length(len: u64) -> Result<(), Error> {
    // 10 MB
    if len > 1024 * 1024 * 10 {
        return Err(Error::new(ErrorKind::InvalidData, "Message too large"));
    }
    Ok(())
}
impl Drop for Server {
    fn drop(&mut self) {
        // Clean up socket file when server is dropped
        let _ = std::fs::remove_file(SOCKET_PATH);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    /// Needs to be run as one test to avoid conflicts due to concurrency
    #[test]
    fn test_all_scenarios() {
        test_client_server_communication();
        test_multiple_clients();
    }

    fn test_client_server_communication() {
        let server_handle = thread::spawn(|| {
            let mut server = Server::new().expect("Failed to create server");

            thread::sleep(Duration::from_millis(100));
            server
                .accept_connection()
                .expect("Failed to accept connections");

            thread::sleep(Duration::from_millis(100));
            let received = server.read_message().expect("Failed to read message");
            let mut result: String = "".to_string();

            received.as_slice().read_to_string(&mut result).unwrap();

            assert_eq!(result, "Hello, Server!");
            server
                .send_message(&"Hello, Client!".to_string().into_bytes())
                .expect("Failed to send message");
        });
        thread::sleep(Duration::from_millis(50));

        let client_handle = thread::spawn(|| {
            let mut client = Client::new().expect("Failed to create client");
            client
                .send_message(&"Hello, Server!".to_string().into_bytes())
                .expect("Failed to send message");
            let response = client.read_message().expect("Failed to read response");
            let mut result: String = "".to_string();

            response.as_slice().read_to_string(&mut result).unwrap();
            assert_eq!(result, "Hello, Client!");
        });
        client_handle.join().expect("Client thread panicked");
        server_handle.join().expect("Server thread panicked");
    }
    fn test_multiple_clients() {
        let mut server = Server::new().expect("Failed to create server");
        let client_handles: Vec<_> = (0..3)
            .map(|i| {
                thread::spawn(move || {
                    thread::sleep(Duration::from_millis(50));
                    let mut client = Client::new().expect("Failed to create client");
                    client
                        .send_message(&format!("Message from client {}", i).into_bytes())
                        .expect("Failed to send message");
                })
            })
            .collect();
        thread::sleep(Duration::from_millis(200));

        for _ in 0..3 {
            server
                .accept_connection()
                .expect("Failed to accept connections");
        }

        assert_eq!(server.connected_clients(), 3);

        for handle in client_handles {
            handle.join().expect("Client thread panicked");
        }
    }
}
