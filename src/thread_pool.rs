use std::{sync::{mpsc::{Sender, Receiver}, atomic::AtomicBool, Arc}, net::TcpStream, time::Duration};

use crate::Request;

pub const MAX_THREADS: usize = env_usize!("MAX_THREADS");
pub const MAX_POOL_SIZE: usize = env_usize!("MAX_POOL_SIZE");

pub struct ThreadPool {
    pub main_thread_tx: Sender<TcpStream>
}
impl ThreadPool {
    pub fn new(on_req: fn(Request), on_err: fn(std::io::Error)) -> Self {
        let mut threads = Vec::with_capacity(MAX_THREADS);
        for _ in 0..MAX_THREADS {
            let (tx, rx): (Sender<TcpStream>, Receiver<TcpStream>) = std::sync::mpsc::channel();
            let _is_ready = Arc::new(AtomicBool::new(false));
            let is_ready = _is_ready.clone();
            std::thread::spawn(move || {
                loop {
                    is_ready.store(true, std::sync::atomic::Ordering::SeqCst);
                    let stream = rx.recv().unwrap();
                    match Request::new(stream) {
                        Ok(req) => on_req(req),
                        Err(e) => on_err(e)
                    }
                }
            });
            threads.push((tx, _is_ready));
        }
        let threads: [(Sender<TcpStream>, Arc<AtomicBool>);MAX_THREADS] = threads.try_into().unwrap();

        let (main_thread_tx, main_thread_rx): (Sender<TcpStream>, Receiver<TcpStream>) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let mut pool: [Option<TcpStream>;MAX_POOL_SIZE] = {
                let mut res = Vec::with_capacity(MAX_POOL_SIZE);
                for _ in 0..MAX_POOL_SIZE {
                    res.push(None);
                }
                res.try_into().unwrap()
            };
            let mut pool_size = 0;
            'l: loop {
                let t = if pool_size > 0 {
                    pool_size -= 1;
                    pool[pool_size].take().unwrap()
                } else {
                    main_thread_rx.recv().unwrap()
                };
                for thread_id in 0..MAX_THREADS {
                    if threads[thread_id].1.load(std::sync::atomic::Ordering::SeqCst) {
                        threads[thread_id].1.store(false, std::sync::atomic::Ordering::SeqCst);
                        threads[thread_id].0.send(t).unwrap();
                        continue 'l;
                    }
                }
                if pool_size < MAX_POOL_SIZE {
                    pool[pool_size] = Some(t);
                    pool_size += 1;
                }
                std::thread::sleep(Duration::from_millis(1))
            }
        });
        
        Self {
            main_thread_tx
        }
    }
    pub fn add_stream(&mut self, stream: TcpStream) {
        self.main_thread_tx.send(stream).unwrap()
    }
}