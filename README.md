## Getting started

#### Installation
rustup - rust update software\
cargo - build tool and package manager
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
cargo --version
```

Check this for IDE support (other tools):
https://www.rust-lang.org/learn/get-started

#### Running
```
cargo run
```

#### Testing
Performance tests should generate results.
```
# run all tests
cargo test
# run perf tests
cargo test --test perf_test
# run unit tests
cargo test --test unit_test
```

#### Tutorial
https://doc.rust-lang.org/book/title-page.html

Must-read: chapter 4, 10.3