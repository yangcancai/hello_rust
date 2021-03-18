mod daemon;
mod lib;
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
fn main() {
    // guess_game::execute();
    data_type::execute();
    functions::execute();
    control_flow::execute();
    struction::execute();
    image::execute();
    enum_struct::execute();
    oop::execute();
    // async_test::execute();
    macro_test::execute();
    // rc_refcell_test::execute();
    time_test::execute();
    demo::execute(false);
    // http_server::execute();
}
