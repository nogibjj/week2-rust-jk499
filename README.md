[![Build binary release](https://github.com/nogibjj/week6-rust-jk499/actions/workflows/release.yml/badge.svg)](https://github.com/nogibjj/week6-rust-jk499/actions/workflows/release.yml)

[![Tests](https://github.com/nogibjj/week6-rust-jk499/actions/workflows/tests.yml/badge.svg)](https://github.com/nogibjj/week6-rust-jk499/actions/workflows/tests.yml)

[![Clippy](https://github.com/nogibjj/week6-rust-jk499/actions/workflows/lint.yml/badge.svg)](https://github.com/nogibjj/week6-rust-jk499/actions/workflows/lint.yml)


# Reverse String.

**Developer:** Jaya Khan 

## Abstract
This Rust program is guessing game. It starts by generating a random secret number between 1 and 100. It then enters a loop where it prompts the user to input a guess. If the user's guess is less than the secret number, the program prints "Too small!". If the user's guess is greater than the secret number, the program prints "Too big!". If the user's guess is equal to the secret number, the program prints "You win!" and exits the loop.


## Requirements
The code was tested on:
- rust edition = 2021


## Project Structure
                                                                               
    reverse-text
    ├── Cargo.toml
    ├── README.md   
    ├── src
        ├── main.rs
    ├── .github
        ├── workflows
            ├── lint.yml
            ├── release.yml
            ├── rustfmt.yml
            ├── tests.yml
    ├── Makefile


## Commands to install Rust
1. sudo apt install curl
2. curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
3. source $HOME/.cargo/env

    ### Check installation:
    rustc --version

    ### Command to uninstall rust
    rustup self uninstall

## Command to build project (CI/CD): 
`make all`