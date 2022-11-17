use std::{sync::{Mutex, Arc}, net::SocketAddr};

#[cfg(test)]
mod tests;

mod static_files;

pub const ADDRESS: &'static str = env!("ADDRESS");

struct Context {
    addrs: Mutex<Vec<SocketAddr>>
}

fn basic_server() {
    println!("Server running on http://{}", ADDRESS);

    let context = Arc::new(Context {
        addrs: Mutex::new(Vec::new()),
    });
    
    gondor_io::GondorIO::new(ADDRESS, context.clone(), |c, mut req| {
        c.addrs.lock().unwrap().push(req.addr);
        if req.method().is_get() {
            if let Err(e) = static_files::send_static_file(&mut req) {
                eprintln!("Error sending static file: {}", e)
            }
        }
    }, |_, e| {
        match e.kind() {
            std::io::ErrorKind::WouldBlock => {}, // Ignore non blocking errors
            _ => eprintln!("Error: {}", e)
        }
    }).unwrap().start_nonblocking();

    for addr in context.addrs.lock().unwrap().iter() {
        println!("Connected addrs: {}", addr);
    }
}

fn main() {
    basic_server()
}