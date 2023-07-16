pub fn send_static_file(req: &mut fsfs::Request) -> Result<(), std::io::Error> {
    match req.path() {
        b"main" | b"css/main.css" => req.send(200, "text/css", include_str!("../public/css/main.css")),
		b"" | b"index" | b"index.html" => req.send(200, "text/html", include_str!("../public/index.html")),
		_ => req.send(404, "text/plain", "NOT FOUND")
    }
}