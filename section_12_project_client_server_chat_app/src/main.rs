/*
Client - Server app
    - Create server that listens to messages on an IP & port
    - Create a client that can connect to the server
    - Client can send message
    - Server receives the message
    - Server sends received message to all connected clients
    - All clients receive the message
    - Client can disconnect
 */

use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// Define some constants
const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn main() {
    // Define the server
    let server = TcpListener::bind(LOCAL).expect("Listener failed to bind...");
    // Set a flag
    server.set_nonblocking(true).expect("Failed to initialize non-blocking");   // Doesn't block the message channel

    // Define a vector for our client to connect to the server
    let mut clients = vec![];
    let (transmitter, receiver) = mpsc::channel::<String>();

    // create a loop for the server to check any messages and send them on to the clients
    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {} connected", addr);
            // Clone sender
            let transmitter = transmitter.clone();
            clients.push(socket.try_clone().expect("Faied to clone client..."));

            thread::spawn(move || loop {
                let mut buff = vec![0; MSG_SIZE];
                match socket.read_exact(&mut buff) {
                    Ok(_) => {
                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        // Convert msg from utf-8 to string
                        let msg = String::from_utf8(msg).expect("Invalid utf-8 message");

                        println!("{}: {:?}", addr, msg);
                        // Send onwards
                        transmitter.send(msg).expect("Failed to send message to receiver...");
                    },
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("Closing connection with {}", addr);
                        break;
                    }
                }

                sleep();
            });
        }

        if let Ok(msg) = receiver.try_recv() {
            clients = clients.into_iter().filter_map(|mut client| {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                client.write_all(&buff).map(|_| client).ok()
            })
                .collect::<Vec<_>>();
        }

        sleep();
    }
}

fn sleep(){
    thread::sleep(Duration::from_millis(100));
}
