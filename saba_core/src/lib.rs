/* 
cargo new saba_core --lib によって作成される
 */ 
#![no_std]

extern crate alloc;

pub mod url;
pub mod http;
pub mod error;