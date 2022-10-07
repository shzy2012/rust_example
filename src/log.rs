#[macro_export]
macro_rules! log {
    ($($args: expr),*) => {
        $(
            print!("[{}] {}:{}: {:?}\n",chrono::prelude::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), file!(), line!(),$args);
        )*
    }
}

#[macro_export]
macro_rules! panick {
    ($($args: expr),*) => {
        $(
            panic!("[{}] {}:{}: {:?}\n",chrono::prelude::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), file!(), line!(),$args);
        )*
    }
}
