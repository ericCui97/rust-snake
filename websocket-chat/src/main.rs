use std::any::Any;
use std::io::{self, ErrorKind, Write, Read};
use std::net::TcpStream;
use std::sync::mpsc::{self,TryRecvError};
use std::thread;
use std::time::Duration;

const LOCAL:&str = "127.0.0.1:6666";
const MSG_SIZE:usize = 32;

fn main(){
    let mut client = TcpStream::connect(LOCAL).expect("client connect error");
    client.set_nonblocking(true).expect("set client non-blocking true");

    let (sender,receiver)=  mpsc::channel::<String>();
    thread::spawn(move||loop{
        let mut buff = vec![0;MSG_SIZE];
        // received server message
        match client.read_exact(&mut buff){
            Ok(_)=>{
                let msg = buff.into_iter().take_while(|&x|x!=0).collect::<Vec<_>>();
                println!("message received {:?}",msg);

            },
            Err(ref err)if err.kind() == ErrorKind::WouldBlock =>(),
            Err(_)=>{
                println!("connection with server error");
                break;
            },

        }

        match receiver.try_recv(){
            Ok(msg)=>{
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE,0);
                client.write_all(&buff).expect("writing to socket failed");
                println!("message sent {:?}",msg);
            },
            Err(TryRecvError::Empty)=>(),
            Err(TryRecvError::Disconnected)=>break
        }
        thread::sleep(Duration::from_millis(300));
    });

    println!("write a message");

    loop {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).expect("reading from stdin failed");
        let msg = buff.trim().to_string();
        if msg == ":quit" || sender.send(msg).is_err() {break}
    }

}