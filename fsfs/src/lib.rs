use std::{net::{TcpListener, ToSocketAddrs}, io::Error};

#[macro_use]
pub mod utils;
pub mod codes;

mod thread_pool;  pub use thread_pool::*;
mod request;      pub use request::*;
mod method;       pub use method::*;
mod file_type;    pub use file_type::*;
mod compiler;     pub use compiler::*;

pub struct Fsfs {
    pub listener: TcpListener,
    pub thread_pool: ThreadPool
}
impl Fsfs {
    pub fn new<C: Clone + Send + 'static>(
        address: impl ToSocketAddrs,
        context: C,
        on_req: fn(&C, Request) -> Result<(), Error>,
        on_err: fn(&C, std::io::Error)
    ) -> Result<Self,std::io::Error> {
        let listener = TcpListener::bind(address)?;
        let thread_pool = ThreadPool::new(context, on_req, on_err);
        Ok(Self {
            listener,
            thread_pool
        })
    }
    pub fn start(self) {
        self.listener.set_nonblocking(false).unwrap();
        loop {
            if let Ok((stream, addr)) = self.listener.accept() {
                self.thread_pool.add_stream(stream, addr)
            }
        }
    }
}