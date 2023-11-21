use ext_php_rs::{php_class, php_impl};

use crate::global_var::get;

#[php_class(name="LDKMP\\App")]
pub struct App;

#[php_impl]
impl App {
    fn __construct() -> Self {
        Self {}
    }

    fn run() -> String {
        format!("Hello, {}!", get("name"))
    }
}
