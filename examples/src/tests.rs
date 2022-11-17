use std::{io::{Write, Read}, sync::{atomic::AtomicU64, Arc}};

pub const ADDRESS: &'static str = "127.0.0.1:7878";
pub const REQUESTS: usize = 1000;

#[test]
fn tests() {
    println!("test");
    std::thread::spawn(|| super::basic_server());
    ok_test();
}

fn ok_test() {
    let mut successful_connections = 0;
    let mut successful_writes = 0;
    let mut successful_reads = 0;
    
    for _ in 0..REQUESTS {
        let conn = std::net::TcpStream::connect(ADDRESS);
        if let Ok(mut conn) = conn {
            if let Ok(_) = conn.write(b"GET /ok HTTP/1.1\r\n\r\ntest") {
                successful_writes += 1;
                let mut res = String::new();
                if let Ok(_) = conn.read_to_string(&mut res) {
                    if res == "HTTP/1.1 200 OK\r\nContent-Length: 2\r\nContent-Type: text/plain\r\n\r\nOK" {
                        successful_reads += 1;
                    }
                }
            }
            successful_connections += 1;
        }
    }

    println!("Requests: {}", REQUESTS);
    println!("Successful connections: {}", successful_connections);
    println!("Successful writes: {}", successful_writes);
    println!("Successful reads: {}", successful_reads);
}