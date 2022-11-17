#[cfg(test)]
mod tests;

mod static_files;

pub const ADDRESS: &'static str = env!("ADDRESS");

fn on_req(mut req: gondor_io::Request) {
    if req.method().is_get() {
        if let Err(e) = static_files::send_static_file(&mut req) {
            eprintln!("Error sending static file: {}", e)
        }
    }
}

fn on_err(e: std::io::Error) {
    match e.kind() {
        std::io::ErrorKind::WouldBlock => {}, // Ignore non blocking errors
        _ => eprintln!("Error: {}", e)
    }
}

fn basic_server() {
    println!("Server running on http://{}", ADDRESS);
    gondor_io::GondorIO::new(ADDRESS, on_req, on_err).unwrap().start();
}

fn main() {
    basic_server()
}