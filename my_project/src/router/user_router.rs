use crate::model::user_model::print_user_model;
use crate::router::test_router::print_test_router as ptest_router;
pub fn print_user_router() {
    println!("user_router");
    print_user_model();
    ptest_router();
    // super::test_router::print_test_router();
}