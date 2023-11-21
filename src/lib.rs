#![cfg_attr(windows, feature(abi_vectorcall))]

mod app;
mod global_var;

use ext_php_rs::prelude::*;
use app::App;

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}
