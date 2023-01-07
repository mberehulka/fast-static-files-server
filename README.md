## fast-static-files-server

### Goal

This is a personal project dedicated to build an api capable of responding http request from the browser with simple static files, using multithreads.

These are the required environment variables:

    MAX_THREADS = "6"
    MAX_BUF_SIZE = "1024"
    STREAM_READ_TIMEOUT = "10"
    STREAM_WRITE_TIMEOUT = "10"
    PUBLIC_DIR = "examples/public"
    COMPILE_TO = "examples/src"
    ROOT_FILE = "index"

This project is a test of a simple, but powerful concept:

- The build script creates an match statment that rapidly "matches" the given path with an write operation of the expected file bytes.

- Using the match statment is much faster than a HashMap or any other method, thanks to the compiler that can apply very specific performance improvements, since both the given value and the result is aways constant.

### Running

Running the basic example:

    cargo run --example basic

Running tests

    cargo test --all-targets -- --show-output

Feel free to use the code as you want.
