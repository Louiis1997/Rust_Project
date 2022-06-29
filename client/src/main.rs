use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use shared::{ChallengeAnswer, ChallengeResult, MD5HashCashOutput, Message, Subscribe};

fn main() {
    let stream = std::net::TcpStream::connect("localhost:7878");
    match stream {
        Ok(mut stream) => {
            write_message(&mut stream, Message::Hello);
            write_message(&mut stream, Message::Subscribe(Subscribe { name: "free_patato".to_string() }));
            loop {
                let response = read_message(&mut stream);
                let response= from_utf8(&response).unwrap();
                println!("{}", response);
                let response = serde_json::from_str(response).unwrap();
                match response {
                    Message::EndOfGame(..) => {
                        break;
                    }
                    Message::Challenge(..) => {
                        write_message(&mut stream, Message::ChallengeResult(
                            ChallengeResult {
                                answer: ChallengeAnswer::MD5HashCash {
                                    0: MD5HashCashOutput {
                                        seed: 12345678,
                                        hashcode: "68B329DA9893E34099C7D8AD5CB9C940".to_string()
                                    },
                                },
                                next_target: "dark_salad".to_string()
                            }))
                    }
                    _ => {}
                }
            }
        }
        Err(err) => panic!("Cannot connect: {err}")
    }

    fn serialize_message(message : Message) -> String {
        let serialized = serde_json::to_string(&message);
        return serialized.unwrap();
    }

    fn write_message(stream: &mut TcpStream, message : Message){
        let serialized = serialize_message(message);
        let size = serialized.len() as u32;
        let size = size.to_be_bytes();
        stream.write_all(&size).unwrap();
        stream.write_all(&serialized.as_bytes()).unwrap();
        let response = read_message(stream);
        println!("{}", from_utf8(&response).unwrap());
    }

    fn read_message(stream: &mut TcpStream) -> Vec<u8> {
        let mut data = [0 as u8; 4];
        match stream.read_exact(&mut data) {
            Ok(_) => {
                let size = u32::from_be_bytes(data) as usize;
                let mut data_res : Vec<u8> = vec![0u8; size.try_into().unwrap()];
                match stream.read_exact(&mut data_res) {
                    Ok(_) => {
                       return data_res;
                    },
                    Err(e) => {
                        panic!("Failed to receive data: {}", e);
                    }
                }
            },
            Err(e) => {
                panic!("Failed to receive data: {}", e);
            }
        }
    }
}
