# 🍅 Tomato Clock (Rust)

[![Test](https://github.com/coolcode/tomato-clock-rs/actions/workflows/test.yml/badge.svg)](https://github.com/coolcode/tomato-clock-rs/actions/workflows/test.yml)
[![Release](https://github.com/coolcode/tomato-clock-rs/actions/workflows/release.yml/badge.svg)](https://github.com/coolcode/tomato-clock-rs/actions/workflows/release.yml)

Tomato Clock is a straightforward command-line Pomodoro application.

**💡 This source code was primarily generated by ChatGPT through a simple conversion from Python to Rust.**

- [Pomodoro Technique](https://en.wikipedia.org/wiki/Pomodoro_Technique)
- [番茄工作法](https://zh.wikipedia.org/zh-cn/%E7%95%AA%E8%8C%84%E5%B7%A5%E4%BD%9C%E6%B3%95)
- [Tomato Clock (Python)](https://github.com/coolcode/tomato-clock)

## Installation

- Install via source code:

```sh
git clone https://github.com/coolcode/tomato-clock-rs.git
cd tomato-clock-rs
cargo build --release
cd target/release
./tomato -t 
```

## How to use

```sh
tomato         # start a 25 minutes tomato clock + 5 minutes break
tomato -t      # start a 25 minutes tomato clock
tomato -t <n>  # start a <n> minutes tomato clock
tomato -b      # take a 5 minutes break
tomato -b <n>  # take a <n> minutes break
tomato -h      # help
```

## Terminal Output

```sh
🍅 tomato 25 minutes. Ctrl+C to exit
 🍅🍅---------------------------------------------- [8%] 23:04 ⏰ 
```

## Desktop Notification

[notify-rust](https://github.com/hoodie/notify-rust)
