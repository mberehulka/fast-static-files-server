use std::{net::{TcpListener, ToSocketAddrs}, sync::{atomic::AtomicBool, Arc}};

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
    pub fn new<C: Clone + Send + 'static>(
        address: impl ToSocketAddrs,
        context: C,
        on_req: fn(&C, Request),
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
        loop {
            if let Ok((stream, addr)) = self.listener.accept() {
                self.thread_pool.add_stream(stream, addr)
            }
        }
    }
    pub fn start_nonblocking(self) {
        let ctrlc_pressed = Arc::new(AtomicBool::new(false));
        let ctrlc_pressed_clone = ctrlc_pressed.clone();
        
        ctrlc::set_handler(move || {
            ctrlc_pressed_clone.store(true, std::sync::atomic::Ordering::SeqCst)
        }).expect("Error setting Ctrl-C handler");

        self.listener.set_nonblocking(true).unwrap();
        loop {
            if ctrlc_pressed.load(std::sync::atomic::Ordering::SeqCst) { return }
            if let Ok((stream, addr)) = self.listener.accept() {
                self.thread_pool.add_stream(stream, addr)
            }
        }
    }
}