use std::ops::Mul;

pub mod data_type {
    pub fn execute() {
        // let i8 = 0;
        // let u8 = 0;
        // let i16 = 0;
        // let u16 = 0;
        // let i32 = 0;
        // let u32 = 0;
        // let i64 = 0;
        // let u64 = 0;
        // let i128 = 0;
        // let u128 = 0;
        // let isize = 0;// arch 32sys equal i32,64sys equal i64
        // let usize = 0;
        // let f32 = 0;
        // let f64 = 0;
        // let f: bool = true;
        //  let c: char = 'c';

        // number type
        //    let decimal = 98_222; // 10进制： 98222
        //    let hex = 0xff; // 16进制： 255
        //    let octal = 0o77;// 8进制: 63
        // let bianry = 0b1111_0000;

        // tuple type
        //  let tup = (1,2,3);
        //  let (x, y, z) = tup;

        // list type
        //  let list = [1,2,3,4];
        //  let list1:[i32;5] = [1,2,3,4,5];
        //  let list2 = [3;5]; // [3,3,3,3,3]
        // let first = list[0]; // 1
        // let second = list[1]; // 2

        // Bitwise operations
        println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
        println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
        println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
        println!("1 << 5 is {}", 1u32 << 5);
        println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

        // Use underscores to improve readability!
        println!("One million is written as {}", 1_000_000u32);
        let _guess: u32 = "42".parse().expect("The string is not integer");
        // println!("decimal = {:?}", first);
    }
}

pub mod functions {
    pub fn execute() {
        let str = String::from("hello");
        let mut ma = 1;
        execute1(&str);
        execute2(&mut ma);
    }
    pub fn execute1(p: &String) {
        println!("p = {}", p)
    }
    pub fn execute2(p: &mut i32) -> i32 {
        *p = *p + 1;
        *p
    }
}

pub mod control_flow {
    pub fn execute() {
        let a = true;
        if a {
            println!("a = true")
        } else {
            println!("else")
        }

        let number = 10;
        if number > 10 {
            println!("number > 10")
        } else if number > 5 && number <= 10 {
            println!("number < 5 && number < 10")
        } else {
            println!("number else")
        }

        let cond = true;
        let result = if cond { 5 } else { 6 };
        println!("reuslt = {}", result);

        loop {
            println!("loop ...");
            break;
        }

        let loop_r = loop {
            break 2;
        };

        let mut i = 0;
        while i < 10 {
            i += 1;
        }

        println!("loop_r = {}", loop_r);

        let list = [1, 2, 3];
        for e in list.iter() {
            print!("{}", e);
        }
        println!();
        // 倒序
        for e in (1..4).rev() {
            print!("{}", e)
        }
    }
}

// 结构体
pub mod struction {
    // 形状接口
    pub trait Shape<T> {
        fn area(&self) -> T;
        fn show(&self) {
            println!("show trait Shape<T>")
        }
    }
    pub trait Animal {
        fn new() -> Self;
    }
    struct Dog {}
    impl Animal for Dog {
        fn new() -> Dog {
            Dog {}
        }
    }
    // 正方形
    struct Rectangle<T> {
        with: T,
        height: T,
    }
    // 圆形
    struct Round<T> {
        r: T,
    }
    impl<T: super::Mul<T, Output = T> + Copy> Shape<T> for Rectangle<T> {
        fn area(&self) -> T {
            return self.with * self.height;
        }
    }
    impl<T: super::Mul<T, Output = T> + Copy> Shape<T> for Round<T> {
        fn area(&self) -> T {
            self.r * self.r
        }
    }
    fn show<T>(s: &impl Shape<T>) {
        s.show();
    }
    fn is_hello<T: AsRef<str>>(s: T) {
        assert_eq!("hello", s.as_ref())
    }

    pub fn execute() {
        let rect = Rectangle {
            with: 1.0,
            height: 1.0,
        };
        let round = Round { r: 10 };
        let round1 = Round { r: 1.1 };
        show(&rect);
        show(&round);
        show(&round1);
        println!(
            "rectangle area = {}, round area = {}, round1 area = {}",
            rect.area(),
            round.area(),
            round1.area()
        );

        let hello = "hello";
        is_hello(hello);
        let hello = "hello".to_string();
        is_hello(hello);
    }
}

pub mod image {
    #[derive(Debug)]
    pub struct Point<'a> {
        x: &'a f32,
        y: &'a f32,
    }
    pub trait Pixel {
        type Subpixel;
        fn new(&self) -> Self::Subpixel;
    }
    pub struct P;
    impl Pixel for P {
        type Subpixel = f32;
        fn new(&self) -> f32 {
            1.0
        }
    }
    impl Pixel for f32 {
        type Subpixel = f32;
        fn new(&self) -> f32 {
            2.0
        }
    }
    /// Iterate over pixel refs.
    #[warn(dead_code)]
    pub struct Pixels<'a, P: Pixel + 'a>
    where
        P::Subpixel: 'a,
    {
        chunks: &'a f32,
        b: &'a P,
    }
    impl<'a, P: Pixel + 'a> Pixel for Pixels<'a, P>
    where
        P::Subpixel: 'a,
    {
        type Subpixel = Point<'a>;
        fn new(&self) -> Point<'a> {
            Point {
                x: self.chunks,
                y: self.chunks,
            }
        }
    }
    pub fn execute() {
        let p = 2.0;
        let a = Pixels {
            chunks: &1.0,
            b: &p,
        };
        println!("{:?}", a.new());
    }
}
pub mod enum_struct {
    /// 枚举类型
    /// Option作为捕获程序失败信息，不实用panic!
    pub fn div(a: i32, b: i32) -> Option<i32> {
        if b == 0 {
            None
        } else {
            Some(a / b)
        }
    }
    #[derive(Debug)]
    pub enum Why {
        ZERO,
    }
    pub type DivResult = Result<i32, Why>;
    pub fn div_r(a: i32, b: i32) -> DivResult {
        if b == 0 {
            Err(Why::ZERO)
        } else {
            Ok(a / b)
        }
    }
    pub fn check_dev(a: i32, b: i32) -> DivResult {
        match div(a, b) {
            None => println!("{} / {} failed !", a, b),
            Some(v) => println!("{} / {} = {}", a, b, v),
        }
        match div_r(a, b) {
            Err(why) => println!("{} / {} failed! why = {:?}", a, b, why),
            Ok(v) => println!("{} / {} = {}", a, b, v),
        }
        println!("div_r({},{})? = {:?}", a, b, div_r(a, b)?);
        div_r(a, b)
    }
    pub fn execute() {
        let _ = check_dev(1, 2);
        let _ = check_dev(1, 0);
        super::checked::op(1.0, 10.0);
    }
}
mod checked {
    #[derive(Debug)]
    enum MathError {
        DivisionByZero,
        NegativeLogarithm,
        NegativeSquareRoot,
    }

    type MathResult = Result<f64, MathError>;

    fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }

    fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    fn ln(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeLogarithm)
        } else {
            Ok(x.ln())
        }
    }

    // 中间函数
    fn op_(x: f64, y: f64) -> MathResult {
        // 如果 `div` “失败” 了，那么返回 `DivisionByZero`
        let ratio = div(x, y)?;

        // 如果 `ln` “失败” 了，那么返回 `NegativeLogarithm`
        let ln = ln(ratio)?;

        sqrt(ln)
    }

    pub fn op(x: f64, y: f64) -> String {
        match op_(x, y) {
            Err(why) => match why {
                MathError::NegativeLogarithm => format!("logarithm of negative number"),
                MathError::DivisionByZero => format!("division by zero"),
                MathError::NegativeSquareRoot => format!("square root of negative number"),
            },
            Ok(value) => format!("{}", value),
        }
    }
}

pub mod oop {
    use std::collections::HashMap;
    pub fn execute() {
        let mut screen = Screen::new();
        screen.install(Box::new(Button));
        screen.install(Box::new(Icon {}));
        screen.install(Box::new(String::from("kkkk")));
        screen.run();
    }
    pub trait Task {
        fn run(&self) -> bool {
            println!("{} run ..", self.name());
            true
        }
        fn name(&self) -> String {
            "kkk".to_string()
        }
    }
    pub struct Timer {
        pub name: String,
    }
    impl Task for Timer {
        fn name(&self) -> String {
            "timer".to_string()
        }
    }
    pub struct Context<T: Task> {
        plugs: HashMap<String, T>,
    }
    impl<T: Task> Context<T> {
        pub fn run(&self) {
            for (_, e) in &self.plugs {
                e.run();
            }
        }
        pub fn install(&mut self, plug: T) {
            self.plugs.insert(plug.name(), plug);
        }
    }

    pub trait Draw {
        fn draw(&self);
    }
    pub struct Button;
    pub struct Icon;
    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Draw for String {
        fn draw(&self) {
            println!("String draw");
        }
    }
    impl Draw for Button {
        fn draw(&self) {
            println!("button draw");
        }
    }
    impl Draw for Icon {
        fn draw(&self) {
            println!("icon draw");
        }
    }
    impl Screen {
        pub fn new() -> Self {
            Screen {
                components: Vec::new(),
            }
        }
        pub fn run(&self) {
            for e in self.components.iter() {
                e.draw()
            }
        }
        pub fn install(&mut self, component: Box<dyn Draw>) {
            self.components.push(component)
        }
    }
}
pub mod async_test {
    use std::{thread, time::Duration};

    use futures::channel::mpsc;
    use futures::executor; //standard executors to provide a context for futures and streams
    use futures::executor::ThreadPool;
    use futures::StreamExt;
    pub fn execute() {
        // let (mut main_sender, mut main_rec) = mpsc::unbounded::<i32>();
        let (main_sender, main_rec) = std::sync::mpsc::channel();
        let thread = thread::spawn(move || {
            let pool = ThreadPool::new().expect("Failed to build pool");
            let (tx, rx) = mpsc::unbounded::<i32>();
            // Create a future by an async block, where async is responsible for an
            // implementation of Future. At this point no executor has been provided
            // to this future, so it will not be running.
            let fut_values = async {
                // Create another async block, again where the Future implementation
                // is generated by async. Since this is inside of a parent async block,
                // it will be provided with the executor of the parent block when the parent
                // block is executed.
                //
                // This executor chaining is done by Future::poll whose second argument
                // is a std::task::Context. This represents our executor, and the Future
                // implemented by this async block can be polled using the parent async
                // block's executor.
                println!("first async block");
                let fut_tx_result = async move {
                    println!("second async block");
                    loop {
                        thread::sleep(Duration::from_millis(1));
                        match main_rec.try_recv() {
                            Ok(_msg) => {
                                println!("future receive terminate !!");
                                tx.unbounded_send(0).expect("Failed to send");
                                break;
                            }
                            Err(_e) => {
                            }
                        }
                    }
                };

                // Use the provided thread pool to spawn the generated future
                // responsible for transmission
                println!("before spawn_ok ");
                pool.spawn_ok(fut_tx_result);
                println!("after spawn_ok ");

                let fut_values = rx.collect();

                // Use the executor provided to this async block to wait for the
                // future to complete.
                fut_values.await
            };

            // Actually execute the above future, which will invoke Future::poll and
            // subsequenty chain appropriate Future::poll and methods needing executors
            // to drive all futures. Eventually fut_values will be driven to completion.
            // thread::sleep(Duration::from_secs(3));
            println!("before executor.");
            let values: Vec<i32> = executor::block_on(fut_values);

            println!("Values={:?}", values);
        });
        let mut line = String::new();
        let stdin = std::io::stdin();
        use std::io::BufRead;
        let _ = stdin.lock().read_line(&mut line);
        main_sender.send(()).unwrap();
        if let Ok(_) = thread.join() {};
        println!("future thread finished!");
    }
}
