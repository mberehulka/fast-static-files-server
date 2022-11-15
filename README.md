## GondorIO

### Goal

This is a personal project dedicated to build an api capable of responding http request from the browser with simple static files, using multithreads.

There is only dependency in this crate, wich is a buid dependency, no deps on the final binary.

It creates an fixed number of threads that receives the TcpStream by an mpsc channel.

Each thread has an AtomicBool that indicates it states, the ThreadPool will use it to choose wich thread should receive the new stream.

If none of the threads are available, the stream is held inside a fixed size vector.

The build script will create an "static_files.rs" file inside the folder set by the COMPILE_TO environment variable.

These are the required environment variables:

    MAX_THREADS = "6"
    MAX_BUF_SIZE = "1024"
    MAX_POOL_SIZE = "1024"
    STREAM_READ_TIMEOUT = "1000"
    STREAM_WRITE_TIMEOUT = "1000"
    PUBLIC_DIR = "examples/public"
    COMPILE_TO = "examples/src"
    ROOT_FILE = "index"

### Running

Run one of the examples with

    cargo run --example example_name

Feel free to use the code as you want.