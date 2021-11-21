# rust examples

https://doc.rust-lang.org/cargo/getting-started/index.html

命令
```bash
cargo update            # updates all dependencies
cargo update -p regex   # updates just “regex”


cargo test
cargo test foo
```

Package Layout
```
.
├── Cargo.lock
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── main.rs
│   └── bin/
│       ├── named-executable.rs
│       ├── another-executable.rs
│       └── multi-file-executable/
│           ├── main.rs
│           └── some_module.rs
├── benches/
│   ├── large-input.rs
│   └── multi-file-bench/
│       ├── main.rs
│       └── bench_module.rs
├── examples/
│   ├── simple.rs
│   └── multi-file-example/
│       ├── main.rs
│       └── ex_module.rs
└── tests/
    ├── some-integration-tests.rs
    └── multi-file-test/
        ├── main.rs
        └── test_module.rs

```

目录解释:
```
Cargo.toml and Cargo.lock are stored in the root of your package (package root).
Source code goes in the src directory.
The default library file is src/lib.rs.
The default executable file is src/main.rs.
Other executables can be placed in src/bin/.
Benchmarks go in the benches directory.
Examples go in the examples directory.
Integration tests go in the tests directory.

If a binary, example, bench, or integration test consists of multiple source files, place a main.rs file along with the extra modules within a subdirectory of the src/bin, examples, benches, or tests directory. The name of the executable will be the directory name.
```


Cargo Home
```
The "Cargo home" functions as a download and source cache.
https://doc.rust-lang.org/cargo/guide/cargo-home.html

config.toml Cargo's global configuration file
credentials.toml Private login credentials from cargo login in order to log in to a registry.
.crates.toml This hidden file contains package information of crates installed via cargo install. Do NOT edit by hand!
```


所有权规则
```bash
Rust的所有权规则保证了同一时刻永远只有一个变量持有一个对象的所有权，避免数据竞争。

1. Rust 中每一个值或者对象都有一个称之为其 所有者（owner）的变量
2. 值或对象有且只能有一个所有者
3. 当所有者离开作用域，所有者所代表的对象或者值会被立即销毁
4. 赋值语句、函数调用、函数返回等会导致所有权转移，原有变量会失效
```

