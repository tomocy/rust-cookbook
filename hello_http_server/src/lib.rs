use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn run(addr: &str) -> Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("server: listen on {}", addr);

    let pool = ThreadPool::new(5)?;

    for stream in listener.incoming() {
        let stream = stream?;
        println!("server: accepted connection");

        pool.execute(|| {
            handle_connection(stream).unwrap();
        });
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut buffer = [0; 512];
    stream.read(&mut buffer)?;

    let (status, filename) = if buffer.starts_with(b"GET / HTTP/1.1\r\n") {
        ("200 OK", "index.html")
    } else if buffer.starts_with(b"GET /sleep HTTP/1.1\r\n") {
        thread::sleep(Duration::from_secs(5));
        ("200 OK", "index.html")
    } else {
        ("404 Not Found", "404.html")
    };

    handle_resource(stream, status, filename)
}

fn handle_resource(mut stream: TcpStream, status: &str, filename: &str) -> Result<()> {
    let mut response = String::new();
    File::open(filename)?.read_to_string(&mut response)?;
    let response = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
        status,
        response.len(),
        response
    );

    stream.write(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    fn new(size: usize) -> Result<ThreadPool> {
        if size < 1 {
            return Err("pool size should be more than 1".into());
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool { workers, sender })
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("thread pool: telling all of the workers to terminate");
        for _ in 0..self.workers.len() {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("thread pool: shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
                println!("thread pool: shutted down worker {}", worker.id);
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = Some(thread::spawn(move || loop {
            let msg = receiver.lock().unwrap().recv().unwrap();
            match msg {
                Message::NewJob(job) => {
                    println!("workder {}: received a job", id);
                    job();
                    println!("workder {}: did the job", id);
                }
                Message::Terminate => {
                    println!("worker {}: was told to terminate", id);
                    break;
                }
            }
        }));

        Worker { id, thread }
    }
}

type Job = Box<dyn FnOnce() + Send>;

enum Message {
    NewJob(Job),
    Terminate,
}
