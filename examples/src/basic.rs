use gondor_io::Request;

mod static_files;

fn on_req(mut req: Request) {
    if req.method().is_get() {
        if let Err(e) = static_files::send_static_file(&mut req) {
            eprintln!("Error sending static file: {}", e)
        }
    }
}

fn on_err(e: std::io::Error) {
    eprintln!("Error: {}", e)
}

fn main() {
    println!("Server running on http://{}", env!("ADDRESS"));
    gondor_io::GondorIO::new(env!("ADDRESS"), on_req, on_err)
        .unwrap()
        .start();
}