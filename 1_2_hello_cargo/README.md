# Hello, Cargo!

Create a new binary project in a new directory:

```
cargo new --bin hello_cargo
cd hello_cargo
```

Build the project:

```
cargo build
```

Run the generated executable:

```
./target/debug/hello_cargo
```

or

```
cargo run
```

Checking the code instead of building it is faster:

```
cargo watch
```

or

```
cargo install cargo-watch
cargo watch -x check
```

`watch --color -n 0,5 "height=$((`stty size | sed 's/ .*//'` - 2)); clear; cargo check --color=always 2>&1 | head -n $height"`

Build for release:

```
cargo build --release
```