use rand::Rng;
use std::cmp::Ordering;
use std::io;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
pub fn execute() {
    guess();
}
// guessing game
fn guess() {
    // genearte random number
    let secret_number: u32 = rand::thread_rng().gen_range(1, 10);
    println!("The random nunber is :{}", secret_number);
    loop {
        let mut guess: String = String::new();
        println!("please input your guess:");
        // will add \n to the end
        io::stdin()
            .read_line(&mut guess)
            .expect("Fail to read line!");
        // remove the \n
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
