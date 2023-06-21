# N Letter Wordle Game
A wordle clone, played in the command line, with words of any length. Also, it gives you the definition of the word when the game is over. 👍

<br/>

<img src="./n-letter-wordle-demo-shorter.gif"/>

<br/>

## Disclaimer
_We are not affiliated at all with Wordle or New York Times. This is simply a toy project for [Jim](github.com/JimLynchCodes) to practice building cli tools and coding in Rust..._

<br/>

## Try The Live, Deployed CLI Tool!

Two different ways to install: via npm or via cargo.

<br/>

### Installation Method 1) NPM

Install `n-letter-wordle` as a global npm dependency:
```sh
npm i -g n-letter-wordle
```

<br/>

### Installation Method 2) Cargo

Install `n-letter-wordle` as a global npm dependency:
```sh
cargo install n-letter-wordle
```
<br/>

## Local Dev
Clone this project, then run it with cargo:
```
cargo run
```

Run unit & integration tests:
```
cargo test
```

Run format & linting checks:
```
cargo fmt
cargo clippy
```

## Deploy

First, login to cargo and npm
```
cargo login
npm adduser
```

Then I deployed with [rust-to-npm](https://github.com/a11ywatch/rust-to-npm):
```
rust-to-npm-cli deploy -b
```


