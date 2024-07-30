# ESP32 shrap memory display implementation

## SETTING UP ENVIRONMENT

Before starting development make sure you have your development environment setup correctly for the target you are building for. A quick introduction to embeded development in rust for the esp32 can be found [here](https://docs.esp-rs.org/book/introduction.html). With everything from setting up your first project, to building and flashing covered, however I will provide a quick rundown on how to get started.

First install esp-idf on your system eather trough espressifs tutorial on the official page or via `cargo`, should matter.

For cargo run:

```
cargo install espup
```

and than

```
espup install
```

Since this is a project that uses the rust `std` library, you will need to install ldproxy aswell:

```
cargo install ldproxy
```

After that you should be good to go.
(If you find any issues with the above provided steps please open an issue)

## BUILDING

**IF YOU ARE ON OSX make sure to run `export CRATE_CC_NO_DEFAULTS=1` before building it will save you precious hours.**

To build the library simply call `cargo build --release`. Witch will build your program in release mode.

To flash it install `espflash` or `cargo install cargo-espflash` with cargo and run the follwing command:

```
cargo espflash flash --monitor
```
