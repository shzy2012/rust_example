# rust examples


命令
```bash
cargo update            # updates all dependencies
cargo update -p regex   # updates just “regex”


cargo test
cargo test foo
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