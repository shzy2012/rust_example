use bitmaps::Bitmap;
use regex::Regex;
use rust_example::bitmap;
use rust_example::concurrency;
use rust_example::inner::{Body, Form, FormParts, Part};
use rust_example::lazy_load::HASHMAP;
use rust_example::log;
use rust_example::sharevariable;
use std::mem;
use std::str;
use std::sync::Arc;
use std::thread;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    log!("log print 1");

    {
        HASHMAP.clone();
    }

    log!("log print 2");

    Ok(())
}
