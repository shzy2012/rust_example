pub mod p1 {

    use futures::{stream, StreamExt}; // 0.3.5
    use reqwest::Client; // 0.10.6
    use std::{ops::Sub, time};

    const CONCURRENT_REQUESTS: usize = 100;

    pub async fn run() {
        let start = time::Instant::now();
        let client = Client::new();

        let urls = vec!["https://qq.com"; 100];

        let bodies = stream::iter(urls)
            .map(|url| {
                let client = &client;
                async move {
                    let resp = client.get(url).send().await?;
                    resp.bytes().await
                }
            })
            .buffer_unordered(CONCURRENT_REQUESTS);

        bodies
            .for_each(|b| async {
                match b {
                    Ok(b) => {
                        println!("Got an error: {}", b.len())
                    }
                    Err(e) => eprintln!("Got an error: {}", e),
                }
            })
            .await;

        let end = time::Instant::now();
        println!("{:?}", end.sub(start).as_secs());
    }
}
// 顺序执行
pub mod p2 {
    use std::{ops::Sub, time};
    use tokio::sync::mpsc;
    use tokio::task;

    pub async fn run() {
        let start = time::Instant::now();
        // 1.初始化channle
        let (tx, mut rx) = mpsc::channel(100);

        // 2.创建任务
        for _ in 0..1 {
            let tx_n = tx.clone();
            task::spawn(async move {
                for _ in 0..100 {
                    let resp = reqwest::get("https://qq.com").await;
                    println!("hello");
                    if let Err(e) = tx_n.send(resp).await {
                        println!("receiver dropped {}", &e.to_string());
                    }
                }
            });
        }

        // drop(tx) 确保任务返回None结果,如果没有drop(tx),则不会返回None
        drop(tx);

        // 3.处理结果
        while let Some(res) = rx.recv().await {
            match res {
                Ok(x) => {
                    println!("got = {:?}", x.status());
                }
                Err(_) => {}
            }
        }

        let end = time::Instant::now();
        println!("{:?}", end.sub(start).as_secs());
    }
}

pub mod p3 {
    use futures::future;
    use reqwest::Client;
    use std::{ops::Sub, time};
    pub async fn run() {
        let start = time::Instant::now();
        let client = Client::new();

        let urls = vec!["https://qq.com"; 100];

        let bodies = future::join_all(urls.into_iter().map(|url| {
            let client = &client;
            async move {
                let resp = client.get(url).send().await?;
                resp.bytes().await
            }
        }))
        .await;

        for b in bodies {
            match b {
                Ok(b) => println!("Got {} bytes", b.len()),
                Err(e) => eprintln!("Got an error: {}", e),
            }
        }

        let end = time::Instant::now();
        println!("{:?}", end.sub(start).as_secs());
    }
}

pub mod p4 {

    use futures::{stream, StreamExt};
    use reqwest::Client;
    use std::{ops::Sub, time};
    use tokio::task;

    const CONCURRENT_REQUESTS: usize = 100;

    pub async fn run() {
        let start = time::Instant::now();
        let client = Client::new();

        let urls = vec!["https://qq.com"; 100];

        let bodies = stream::iter(urls)
            .map(|url| {
                let client = client.clone();
                task::spawn(async move {
                    let resp = client.get(url).send().await?;
                    resp.bytes().await
                })
            })
            .buffer_unordered(CONCURRENT_REQUESTS);

        bodies
            .for_each(|b| async {
                match b {
                    Ok(Ok(b)) => {
                        println!("Got {} bytes", b.len());
                    }
                    Ok(Err(e)) => eprintln!("Got a reqwest::Error: {}", e),
                    Err(e) => eprintln!("Got a tokio::JoinError: {}", e),
                }
            })
            .await;

        let end = time::Instant::now();
        println!("{:?}", end.sub(start).as_secs());
    }
}
