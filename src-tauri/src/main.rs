// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// mod mymacos;
mod hello;
mod mymacos;

fn main() {
    
    test2_lib::output1();
    hello::output1();
    mymacos::output1();
    test2_lib::run();


}
