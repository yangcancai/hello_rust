mod lib;

use lib::async_test;
use lib::control_flow;
use lib::data_type;
use lib::functions;
use lib::struction;
use lib::{enum_struct, image, oop};
fn main() {
    // guess_game::execute();
    data_type::execute();
    functions::execute();
    control_flow::execute();
    struction::execute();
    image::execute();
    enum_struct::execute();
    oop::execute();
    async_test::execute();
    // http_server::execute();
}
