# rust-arduino-nano33-iot-boilerplate

A boilerplate project to give you a fresh start for your arduino applications in rust. I prefer to use `RTIC`([info](https://rtic.rs/0.5/book/en/)) as 'Hello World!' application. You are of course free to use whatever you want.

## Prerequisites
- Install [cargo-binutils](https://github.com/rust-embedded/cargo-binutils).
- Install [arduino-cli](https://github.com/arduino/arduino-cli).
- Install [cargo-make](https://github.com/sagiegurari/cargo-make).
- Install target for `nano33-iot` board. Run following command to do it.
```sh
rustup target add thumbv6m-none-eabi
```

## Build
Building is fairly simple, you just need to run `cargo build` as usual.

## Flash
To flash the application to MCU, I have created a makefile for cargo-make so you can just run following command while your arduino in bootloader mode.

```sh
cargo make flash
```

### IMPORTANT!!
**After flashing, serial communication between your arduino and pc will be gone. So if you want to put your arduino into bootloader mode you just need to press `rst` button on the board twice.**
Then you will be able to flash your application to arduino again.

