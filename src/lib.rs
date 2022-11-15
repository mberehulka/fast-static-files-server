use std::net::{TcpListener, ToSocketAddrs};

#[macro_use]
pub mod utils;
pub mod codes;

mod thread_pool;
pub use thread_pool::*;
mod request;
pub use request::*;
mod method;
pub use method::*;

pub struct GondorIO {
    pub listener: TcpListener,
    pub thread_pool: ThreadPool
}
impl GondorIO {
    pub fn new(
        address: impl ToSocketAddrs,
        on_req: fn(Request),
        on_err: fn(std::io::Error)
    ) -> Result<Self,std::io::Error> {
        let listener = TcpListener::bind(address)?;
        let thread_pool = ThreadPool::new(on_req, on_err);
        Ok(Self {
            listener,
            thread_pool
        })
    }
    pub fn start(mut self) {
        for stream in self.listener.incoming() {
            self.thread_pool.add_stream(stream.unwrap());
        }
    }
}