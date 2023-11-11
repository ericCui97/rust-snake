use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

const LOCAL: &str = "127.0.0.1:6666";
const MSG_SIZE: u8 = 32;

fn sleep() {
    thread::sleep(::std::time::Duration::from_millis(100));
}

fn main() {
    let server = TcpListener::bind(LOCAL).expect("listener failed to bind");
    println!("> listening {}",LOCAL);
    server.set_nonblocking(true).expect("failed to initialize non-blocking");
    let mut clients = vec![];
    let (sender, receiver) = mpsc::channel();

    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            println!("client {} connected", addr);
            let sender = sender.clone();
            clients.push(socket.try_clone().expect("failed to clone client"));
            thread::spawn(move || loop {
                let mut buff = vec![0, MSG_SIZE];

                match socket.read_exact(&mut buff) {
                    Ok(_) => {
                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let msg = String::from_utf8(msg).expect("invalid utf8 message");

                        println!("{}:{:?}", addr, msg);
                        sender.send(msg).expect("failed to send message to receiver");
                    }
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("closing connection with: {}", addr);
                        break;
                    }
                }
                // wait 100ms;
                sleep();
            });
        }
        if let Ok(msg) = receiver.try_recv(){
            clients = clients.into_iter().filter_map(|mut client| {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE as usize, 0);
                client.write_all(&buff).map(|_| client).ok()
            }).collect::<Vec<_>>();
        }
        sleep();

    }
}
