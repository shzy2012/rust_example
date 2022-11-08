use bitmaps::Bitmap;
use regex::Regex;
use rust_example::bitmap;
use rust_example::concurrency;
use rust_example::inner::{Body, Form, FormParts, Part};
use rust_example::lazy_load::HASHMAP;
use rust_example::log;
use rust_example::sharevariable;
use std::collections::HashMap;
use std::mem;
use std::str;
use std::sync::Arc;
use std::thread;
use std::vec;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    log!("log print 1");

    {
        HASHMAP.clone();
    }

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
        }
    });

    log!("log print 2");

    Ok(())
}

pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => {
            format!("{}{}", first.to_uppercase(), c.collect::<String>())
        }
    }
}
