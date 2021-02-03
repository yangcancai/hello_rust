use std::io::BufRead;
use std::{
    fs,
    sync::{mpsc, Arc, Mutex},
};
use std::{io, thread};
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    time::Duration,
};
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

enum Message {
    NewJob(Job),
    Terminate,
}
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // error code using while let
            // while let Ok(job) = receiver.lock().unwrap().recv(){
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("worker {} was told terminate.", id);
                    break;
                }
            }
        });
        Worker {
            id: id,
            thread: Some(thread),
        }
    }
}
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}
type Job = Box<dyn FnOnce() + Send + 'static>;
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Send Terminate to all workers!");
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all workers!");
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                if let Ok(_) = thread.join(){}
            }
        }
    }
}
pub fn execute() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    let pool = ThreadPool::new(4);
    listener
        .set_nonblocking(true)
        .expect("Cannot set non-blocking");
    let (sender, receiver)  = mpsc::channel();
    let thread = thread::spawn(move || {
    for stream in listener.incoming(){
        match stream {
            Ok(s) => {
                pool.execute(|| {
                    handle_connection(s);
                });
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(1));
                // was told terimnate.
                if let Ok(_) = receiver.try_recv(){
                        break;
                };
                // continute
                continue;
            }
            Err(e) => panic!("encountered IO error: {}", e),
        }
    }});
    let mut line = String::new();
    let stdin = io::stdin();
    let _ = stdin.lock().read_line(&mut line);
    println!("Main thread: Shutting down.");
    sender.send(()).unwrap();
    // waits for other threads finish 
    if let Ok(_) = thread.join(){};
    println!("Main thread: the associated thread was finished.");
}
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Request:{}", String::from_utf8_lossy(&buffer[..]));
    let index = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status_line, file) = if buffer.starts_with(index) {
        ("HTTP/1.1 200 OK", "http_server/static/index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "http_server/static/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "http_server/static/404.html")
    };
    let content = fs::read_to_string(file).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        content.len(),
        content
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
