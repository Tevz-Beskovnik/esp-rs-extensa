# ESP32 shrap memory display implementation

## Setting up environment

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

## Building

**IF YOU ARE ON OSX make sure to run `export CRATE_CC_NO_DEFAULTS=1` before building or running any of the bellow scripts it will save you precious hours.**

After that you want to use the provided scripts for flashing the file system or app.

**NOTE:**

**When following the bellow steps do not disconet the serial cable, it will brick your device**

## Flashing the app

To flash the app run the following command from home directory:

```sh
chmod +x ./scripts/flash_app.sh
./scripts/flash_app.sh release monitor
````

This will build the app in release mode, flash it and open up the serial monitor.

## Flashing the filesystem

Before flashing the file system, please check that your flash is big enough to use the flash configuration provided, ideally 2MB should be enough.

Run the following command to flash the filesystem:

```sh
chmod +x ./scripts/flash_spiffs.sh
./scripts/flash_spiffs.sh
```

## Monitoring

To monitor the chip trought serial run:

```sh
espflash monitor
```

***PRO TIP: before adding execution privilages to any file and running it, it is advised you look over it once or twice to make sure you are not installing malware***
