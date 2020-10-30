
pub mod data_type{
    pub fn execute(){
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
        let _guess: u32 = "42".parse().expect("The string is not integer");
        // println!("decimal = {:?}", first);
    }
}

pub mod functions{
    pub fn execute(){
        let str = String::from("hello"); 
        let mut ma = 1;
        execute1(&str);
        execute2(& mut ma);
    }
    pub fn execute1(p: &String){
        println!("p = {}", p)
    }
    pub fn execute2(p: & mut i32) -> i32 {
         *p = *p + 1;
         *p
    }
}


pub mod control_flow{
    pub fn execute(){
        let a = true;
        if a {                
            println!("a = true")
        }else{                
            println!("else")
        }

        let number = 10;
        if number > 10 {                
            println!("number > 10")
        }else if number > 5 && number <= 10{
            println!("number < 5 && number < 10")
        }else{
            println!("number else")
        }

        let cond = true;
        let result = if cond {5} else {6};
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



        let list = [1,2,3];
        for e in list.iter(){
            print!("{}", e);
        }
        println!();
        // 倒序
        for e in (1..4).rev() {
            print!("{}", e)
        }
    }
}