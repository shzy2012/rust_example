pub mod t1 {
    use std::sync::{Arc, Mutex};
    use tokio::task;

    // 不可变
    pub async fn test1() {
        struct Foo(i32);
        let foo = std::sync::Arc::new(Foo(1));
        {
            let foo = foo.clone();
            task::spawn(async move {
                println!("{}", foo.0);
            });
        }

        println!("{}", foo.0);
    }

    pub async fn test2() {
        struct Foo(i32);
        let foo = Foo(0);
        let mutex = Mutex::new(foo);
        let arc = Arc::new(mutex);
        let child;

        {
            let arc = arc.clone();
            child = task::spawn(async move {
                let mut guard = arc.lock().unwrap();
                guard.0 += 1;
            });
        }
        {
            let mut guard = arc.lock().unwrap();
            guard.0 += 1;
        }
        child.await;
        let mut guard = arc.lock().unwrap();
        println!("{}", guard.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::libs::sharevariable::t1;

    #[tokio::test]
    async fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}

// cargo test libs::sharevariable::tests::exploration -- --exact
