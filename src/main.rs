mod basic;
mod io_path;
mod link_list;

#[allow(unused_imports)]
use crate::basic::run_basic_mod;
#[allow(unused_imports)]
use crate::io_path::run_file_io;
#[allow(unused_imports)]
use crate::link_list::run_link_list;

fn main() {
    // run_basic_mod();
    // run_link_list();
    run_file_io();
}
