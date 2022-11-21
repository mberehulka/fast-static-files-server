use std::{time::Duration, net::{TcpStream, SocketAddr}, io::{Read, Write}};

use crate::Method;

pub const MAX_BUF_SIZE: usize = env_usize!("MAX_BUF_SIZE");
pub const STREAM_READ_TIMEOUT: Option<Duration> = Some(Duration::from_millis(env_u64!("STREAM_READ_TIMEOUT")));
pub const STREAM_WRITE_TIMEOUT: Option<Duration> = Some(Duration::from_millis(env_u64!("STREAM_WRITE_TIMEOUT")));

pub struct Request {
    pub stream: TcpStream,
    pub addr: SocketAddr,
    pub buffer: [u8;MAX_BUF_SIZE],
    pub buffer_len: usize
}
impl Request {
    pub fn new(mut stream: TcpStream, addr: SocketAddr) -> Result<Self, std::io::Error> {
        if let Err(e) = stream.set_read_timeout(STREAM_READ_TIMEOUT) { return Err(e) }
        if let Err(e) = stream.set_write_timeout(STREAM_WRITE_TIMEOUT) { return Err(e) }
        let mut buffer = [0;MAX_BUF_SIZE];
        let mut buffer_len = 0;
        loop {
            match stream.read(&mut buffer) {
                Ok(len) => {
                    buffer_len += len;
                    if len < MAX_BUF_SIZE || buffer_len >= MAX_BUF_SIZE { break }
                },
                Err(e) => return Err(e)
            }
        }
        if let Err(e) = stream.flush() { return Err(e) }
        Ok(
            Self {
                stream,
                addr,
                buffer,
                buffer_len
            }
        )
    }
    pub fn method(&self) -> Method {
        let mut i = 0;
        loop {
            if self.buffer[i] != b' ' && i < self.buffer_len {
                i += 1;
            }else {
                break
            }
        }
        match &self.buffer[0..i] {
            b"GET" => Method::GET,
            b"POST" => Method::POST,
            b"PATCH" => Method::PATCH,
            b"DELETE" => Method::DELETE,
            b"PUT" => Method::PUT,
            _ => Method::UNKNOW
        }
    }
    pub fn path<'a>(&'a self) -> &'a [u8] {
        let mut j = 0;
        while self.buffer[j] != b' ' && j < self.buffer_len {
            j += 1;
        }
        j += 2;
        let mut i = j;
        while self.buffer[i] != b' ' && i < self.buffer_len {
            i += 1;
        }
        &self.buffer[j..i]
    }
    pub fn path_queries<'a>(&'a self) -> (&'a [u8], Vec<[&'a [u8];2]>) {
        let mut j = 0;
        while self.buffer[j] != b' ' && j < self.buffer_len {
            j += 1;
        }
        j += 2;
        let mut i = j;
        while self.buffer[i] != b' ' && self.buffer[i] != b'?' && i < self.buffer_len {
            i += 1;
        }
        i -= 2;
        let mut queries = Vec::new();
        let mut qi = i + 3;
        while qi < self.buffer_len {
            let ni = qi;
            let mut nf = qi;
            while self.buffer[nf] != b'=' && nf < self.buffer_len {
                nf += 1;
            }
            qi = nf;
            while self.buffer[qi] != b'&' && qi < self.buffer_len {
                qi += 1;
            }
            queries.push([
                &self.buffer[ni..nf],
                &self.buffer[(nf+1)..qi]
            ]);
            qi += 1;
        }
        (
            &self.buffer[j..i],
            queries
        )
    }
    pub fn send(&mut self, code: usize, conttype: impl AsRef<[u8]>, content: impl AsRef<[u8]>) -> Result<(),std::io::Error> {
        let contlen = content.as_ref().len();
        let codestr = crate::codes::code(code);
        write!(self.stream, "HTTP/1.1 {code} {codestr}\r\nContent-Length: {contlen}\r\nContent-Type: ")?;
        self.stream.write(conttype.as_ref())?;
        self.stream.write(b"\r\n\r\n")?;
        self.stream.write(content.as_ref())?;
        Ok(())
    }
}