use std::{io::Write, path::{Path, PathBuf}};

pub const PUBLIC_DIR: &'static str = env!("PUBLIC_DIR");
pub const COMPILE_TO: &'static str = env!("COMPILE_TO");
pub const ROOT_FILE: &'static str = env!("ROOT_FILE");

pub fn compile() {
    let public_dir = Path::new(PUBLIC_DIR);
    std::fs::create_dir_all(public_dir).unwrap();
    
    let mut paths = Vec::new();
    pathloop(public_dir, &mut paths);
    let mut file = std::fs::OpenOptions::new()
        .create(true).truncate(true).write(true)
        .open(Path::new(COMPILE_TO).join("static_files.rs")).unwrap();
    
    let pathdiff = pathdiff::diff_paths(PUBLIC_DIR, COMPILE_TO).unwrap();
    let prefix = pathdiff.to_string_lossy().replace('\\',"/");
    let mut routes = String::new();
    for path in &paths {
        let path = path.strip_prefix(PUBLIC_DIR).unwrap();
        let with_ext = path.with_extension("");
        let name = with_ext.file_name().unwrap().to_string_lossy();
        let filetype = crate::file_type(path.extension().unwrap().to_str().unwrap());
        let path = path.to_string_lossy().replace('\\', "/");
        if name == ROOT_FILE {
            routes.push_str(&format!(
                "b\"\" | b\"{name}\" | b\"{path}\" => req.send(200, \"{filetype}\", include_str!(\"{prefix}/{path}\")),\n\t\t"))
        }else {
            routes.push_str(&format!(
                "b\"{name}\" | b\"{path}\" => req.send(200, \"{filetype}\", include_str!(\"{prefix}/{path}\")),\n\t\t"))
        }
    }

write!(file, "pub fn send_static_file(req: &mut fsfs::Request) -> Result<(), std::io::Error> {{
    match req.path() {{
        {routes}_ => req.send(404, \"text/plain\", \"NOT FOUND\")
    }}
}}").unwrap();
}
fn pathloop(path: &std::path::Path, paths: &mut Vec<PathBuf>) {
    for path in std::fs::read_dir(path).unwrap() {
        let path = path.unwrap().path();
        if path.is_dir() {
            pathloop(path.as_path(), paths)
        } else if path.is_file() {
            paths.push(path.clone());
            println!("cargo:debug=Static file: {}", path.display())
        }
    }
}