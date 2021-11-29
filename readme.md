# rust examples

https://doc.rust-lang.org/cargo/getting-started/index.html

https://rustwiki.org/zh-CN/rust-by-example/index.html

命令
```bash
rustup update         Get the latest version of Rust
       show           Show the active and installed toolchains or profiles
       update         Update Rust toolchains and rustup
       check          Check for updates to Rust toolchains and rustup
       default        Set the default toolchain
       toolchain      Modify or query the installed toolchains
       target         Modify a toolchain's supported targets
       component      Modify a toolchain's installed components
       override       Modify directory toolchain overrides
       run            Run a command with an environment configured for a given toolchain
       which          Display which binary will be run for a given command
       doc            Open the documentation for the current toolchain
       man            View the man page for a given command
       self           Modify the rustup installation
       set            Alter rustup settings
       completions    Generate tab-completion scripts for your shell
       help           Prints this message or the help of the given subcommand(s)

cargo new project
cargo update            # updates all dependencies
cargo update -p regex   # updates just “regex”
cargo test
cargo test foo
      build, b    Compile the current package
      check, c    Analyze the current package and report errors, but don't build object files
      clean       Remove the target directory
      doc, d      Build this package's and its dependencies' documentation
      new         Create a new cargo package
      init        Create a new cargo package in an existing directory
      run, r      Run a binary or example of the local package
      test, t     Run the tests
      bench       Run the benchmarks
      update      Update dependencies listed in Cargo.lock
      search      Search registry for crates
      publish     Package and upload this package to the registry
      install     Install a Rust binary. Default location is $HOME/.cargo/bin
      uninstall   Uninstall a Rust binary

编译 rustc
格式化 rustfmt
    rustup component add rustfmt #安装
    cargo fmt  #格式化
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

函数表
```rust
std::any::type_name::<T>()    //返回变量类型名称
std::mem::size_of_val(&str))  //返回一个变量所占的字节数
std::mem
```

属性
```rust
#[allow(dead_code)] #死代码，无效代码
#[warn(dead_code)]
```

----------
常见问题

1. no authentication available
```bash
eval `ssh-agent -s`
ssh-add  # 把专用密钥添加到ssh-agent的高速缓存中
cargo build
```

