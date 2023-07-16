mod static_files;

fn main() {
    println!("Server running on http://{}", env!("ADDRESS"));
    fsfs::Fsfs::new(env!("ADDRESS"), (), |_, mut req| {
        if req.method().is_get() {
            static_files::send_static_file(&mut req)
        } else {
            req.send(404, "text/plain", "NOT FOUND")
        }
    }, |_, e| panic!("{e}")).unwrap().start()
}