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
    pub fn start(self) {
        let ctrlc_pressed = Arc::new(AtomicBool::new(false));
        let ctrlc_pressed_clone = ctrlc_pressed.clone();
        
        ctrlc::set_handler(move || {
            ctrlc_pressed_clone.store(true, std::sync::atomic::Ordering::SeqCst)
        }).expect("Error setting Ctrl-C handler");

        println!("Press Ctrl-C to gracefully shutdown ...");
        
        self.listener.set_nonblocking(true).unwrap();
        loop {
            if ctrlc_pressed.load(std::sync::atomic::Ordering::SeqCst) { return }
            if let Ok((stream, addr)) = self.listener.accept() {
                self.thread_pool.add_stream(stream, addr)
            }
        }
    }
}