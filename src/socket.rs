use regex::Regex;
use std::io::{Read, Write};
use std::net::TcpStream;

pub struct Socket {
    stream: TcpStream,
}

impl Socket {
    pub fn new(host: &str) -> Self {
        let stream = TcpStream::connect(host).unwrap();
        Self { stream }
    }

    pub fn send(&mut self, data: &[u8]) {
        self.stream.write_all(data).unwrap();
        self.stream.flush().unwrap();
    }

    pub fn sendline(&mut self, data: &[u8]) {
        self.send(&[data, b"\n"].concat())
    }

    pub fn recv(&mut self, size: usize) -> Vec<u8> {
        let mut result = vec![0u8; size];
        self.stream.read(&mut result).unwrap();
        result
    }

    pub fn recvuntil(&mut self, end: &str) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];
        loop {
            result.append(&mut self.recv(1));
            if result.ends_with(&end.as_bytes()) {
                break;
            }
        }
        result
    }

    pub fn recvline(&mut self) -> Vec<u8> {
        self.recvuntil("\n")
    }

    pub fn recvregex(&mut self, regex: &str) -> String {
        let re = Regex::new(regex).unwrap();

        loop {
            let result = self.recvline();
            let string = String::from_utf8_lossy(&result);

            match re.captures(&string) {
                Some(captures) => {
                    return captures.get(1).unwrap().as_str().to_string();
                }
                None => continue,
            }
        }
    }
}
