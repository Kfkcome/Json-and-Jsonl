// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod utils;

use utils::WinControlButtonPos;

fn main() {
    
    test2_lib::output1();
    WinControlButtonPos::output1();
    test2_lib::run();

}
