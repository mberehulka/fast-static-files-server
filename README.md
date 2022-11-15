## GondorIO

### Goal

This is a personal repo dedicated to response http with simple static files responses, with multithreading.

There is only dependency in this crate, and it is a simple buid dependency, no deps on the final binary.

### Running

See the example [config.toml](./.cargo/config.toml) for all the environment variables needed.

The build script will create an "static_files.rs" file inside the folder set by the COMPILE_TO environment variable.

Run each example with

    cargo run --example example_name