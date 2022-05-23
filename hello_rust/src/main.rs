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
    // rabbit::execute();
    println!(
        "{:?},{:?},{:?},{:?},{:?}",
        Row::Value(0),
        Row::Value("aa"),
        Row::Value(1),
        Row::Value(vec![0, 1, 2, 3]),
        Row::Value(1).is_integer()
    );
    let s1 = String::from("kkk");
}

struct Ref;

struct Container<'a> {
  r : &'a Ref
}

struct ContainerB<'a> {
  c : Container<'a>
}

trait ToC<'a> {
  fn to_c<'b>(&self, r : &'b Ref) -> &Container<'a>;
}
impl <'a> ToC<'a> for ContainerB<'a> {
    fn to_c<'b>(&self, _r: &'b Ref) -> &Container<'a> {
       &self.c 
    }
}

#[derive(Debug)]
enum Row<T> {
    Value(T),
}
use std::any::type_name;
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
impl<T> Row<T> {
    pub fn is_integer(self) -> bool {
        let Row::Value(value) = self;
        match type_of(value) {
            "i8" | "i16" | "i32" | "i64" | "i128" | "u8" | "u16" | "u32" | "u64" | "u128" => true,
            o => {
                println!("{}", o);
                false
            }
        }
    }
}
