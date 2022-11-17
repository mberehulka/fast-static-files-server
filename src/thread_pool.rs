use std::{sync::{mpsc::{Sender, Receiver}, Arc, Mutex}, net::{TcpStream, SocketAddr}, thread::JoinHandle};

use crate::Request;

pub const MAX_THREADS: usize = env_usize!("MAX_THREADS");

pub struct ThreadPool {
    pub tx: Sender<Option<(TcpStream, SocketAddr)>>,
    pub threads: Vec<JoinHandle<()>>
}
impl ThreadPool {
    pub fn new<C: Clone + Send + 'static>(context: C, on_req: fn(&C, Request), on_err: fn(&C, std::io::Error)) -> Self {
        let mut threads = Vec::with_capacity(MAX_THREADS);
        let (tx, rx): 
            (Sender<Option<(TcpStream, SocketAddr)>>, Receiver<Option<(TcpStream, SocketAddr)>>)
            = std::sync::mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        for _ in 0..MAX_THREADS {
            let rx = rx.clone();
            let context = context.clone();
            threads.push(std::thread::spawn(move || {
                loop {
                    let rx = rx.lock().unwrap();
                    let stream = rx.recv();
                    drop(rx);
                    match stream.unwrap() {
                        Some((stream, addr)) => match Request::new(stream, addr) {
                            Ok(req) => on_req(&context, req),
                            Err(e) => on_err(&context, e)
                        },
                        None => break
                    }
                }
            }));
        }
        Self {
            tx, threads
        }
    }
    #[inline]
    pub fn add_stream(&self, stream: TcpStream, addr: SocketAddr) {
        self.tx.send(Some((stream, addr))).unwrap()
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in 0..MAX_THREADS {
            self.tx.send(None).unwrap()
        }
        for _ in 0..MAX_THREADS {
            self.threads.remove(0).join().unwrap();
        }
    }
}