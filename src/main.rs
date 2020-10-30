mod lib;
use lib::functions;
use lib::data_type;
use lib::control_flow;
fn main() {
    // guess_game::execute();
     data_type::execute();
     functions::execute();
     control_flow::execute();
     let a = 1;
     print!("a = {}, &a = {}", a, *(&a))
}