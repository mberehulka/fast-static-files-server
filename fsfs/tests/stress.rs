use std::{io::{Write, Read}, time::Instant};

pub const ADDRESS: &'static str = "127.0.0.1:7878";
pub const REQUESTS: usize = 5000;

#[test]
fn stress() {
    std::thread::spawn(|| {
        fsfs::Fsfs::new(ADDRESS, (), |_, mut req| {
            req.send(200, "text/plain", "OK")
        }, |_, e| panic!("{e}")).unwrap().start()
    });

    let start = Instant::now();
    
    for _ in 0..REQUESTS {
        let conn = std::net::TcpStream::connect(ADDRESS);
        if let Ok(mut conn) = conn {
            if let Ok(_) = conn.write(b"GET /ok HTTP/1.1\r\n\r\ntest") {
                let mut res = String::new();
                if let Ok(_) = conn.read_to_string(&mut res) {
                    assert!(res == "HTTP/1.1 200 OK\r\nContent-Length: 2\r\nContent-Type: text/plain\r\n\r\nOK")
                }
            }
        }
    }

    println!("{REQUESTS} requests in {} s", (Instant::now() - start).as_secs_f32());
}