mod config;
mod router;
mod model;
fn main() {

    config::print_config();

    router::user_router::print_user_router();
    model::user_model::print_user_model();

    lib::lib_test::print_lib();
    println!("main");
}


