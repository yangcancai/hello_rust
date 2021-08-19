mod daemon;
mod lib;
use std::ops::Deref;

use daemon::demo;
use futures::executor;
use hello_rust::rc_refcell_test;
use lib::async_test;
use lib::control_flow;
use lib::data_type;
use lib::functions;
use lib::macro_test;
use lib::struction;
use lib::time_test;
use lib::{enum_struct, image, oop};

// #[get("/")]
// fn hello1() -> &'static str {
// "Hello, world!"
// }
// #[rocket::main]

fn hello(a: &str) {
    println!("{}", a)
}
use std::io::{self, BufRead};
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

fn run() {
    let (tx, rx) = mpsc::channel();
    let thread = thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(1));
        match rx.recv() {
            Ok(msg) => {
                if msg == "stop" {
                    break;
                }
                println!("do task => {}", msg);
            }
            Err(_recverror) => {
                break;
            }
        }
    });
    let stdin = io::stdin();
    loop {
        let mut line = String::new();
        let _ = stdin.lock().read_line(&mut line);
        match &*line {
            "stop\n" => {
                tx.send(line.trim().to_string()).unwrap();
                break;
            }
            _ => tx.send(line.trim().to_string()).unwrap(),
        }
    }
    if let Ok(_) = thread.join() {};
    println!("Main thread: the associated thread was finished.")
}
fn main() {
    // guess_game::execute();
    // run();
    // data_type::execute();
    // functions::execute();
    // control_flow::execute();
    // struction::execute();
    // image::execute();
    // enum_struct::execute();
    // oop::execute();
    // // async_test::execute();
    // macro_test::execute();
    // // rc_refcell_test::execute();
    // time_test::execute();
    // demo::execute(false);
    // http_server::execute();
    // data_structure::execute();
    // alloc::execute();
    rabbit::execute();
}
