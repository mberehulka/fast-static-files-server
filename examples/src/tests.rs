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
    let successful_connections = Arc::new(AtomicU64::new(0));
    let successful_writes = Arc::new(AtomicU64::new(0));
    let successful_reads = Arc::new(AtomicU64::new(0));
    
    for _ in 0..REQUESTS {
        let conn = std::net::TcpStream::connect(ADDRESS);
        if let Ok(mut conn) = conn {
            if let Ok(_) = conn.write(b"GET /ok HTTP/1.1\r\n\r\ntest") {
                successful_writes.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                let mut res = String::new();
                if let Ok(_) = conn.read_to_string(&mut res) {
                    if res == "HTTP/1.1 200 OK\r\nContent-Length: 2\r\nContent-Type: text/plain\r\n\r\nOK" {
                        successful_reads.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    }
                }
            }
            successful_connections.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
    }

    println!("Requests: {}", REQUESTS);
    println!("Successful connections: {}", successful_connections.load(std::sync::atomic::Ordering::Relaxed));
    println!("Successful writes: {}", successful_writes.load(std::sync::atomic::Ordering::Relaxed));
    println!("Successful reads: {}", successful_reads.load(std::sync::atomic::Ordering::Relaxed));
}