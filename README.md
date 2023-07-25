![Version 0.1](https://img.shields.io/badge/Version%200.1-FFC832?style=for-the-badge&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-000?style=for-the-badge&logo=rust&logoColor=white)
[![MIT License](https://img.shields.io/badge/MIT%20License-004772?style=for-the-badge&logo=license&logoColor=white)](https://github.com/b1rd-dev/rust_grep/blob/main/LICENSE.md)

## Small console Grep with findings highlighting written on rust

## Installation

Download source code from github: 

`git clone https://github.com/lnB51/rust_grep`

Run the project using Cargo:

`cargo run [word for searching] [options] [target]`

<br />

### Paremeters:

<hr/>

#### [word for searching]

##### Any word for search in `[target]`, for search string use `"string for search"`

#### Exaples:
`cargo run hello [options] [target]`
`cargo run "hello world" [options] [target]`

<hr/>

#### [options]

##### Option for search(search type), currently available parameters: `-f` for searching in a file and `-s` for searching in a string.

#### Exaples:
`cargo run hello -f myfile.txt`
`cargo run hello -s "Hello world"`

<hr/>

#### [target]

##### Target for searching in, if selected `[option]` is `-f` - taget must be a file, if selected `[option]` is `-s` - must be a string.

#### Exaples:
`cargo run hello -f myfile.txt`
`cargo run hello -s "Hello world, searching method in string"`

<hr/>

<br />

### Searching case sensitivity can be changed by adding `CASE_INSENSITIVE=true` to your terminal env.

#### Case sensitive mode: `cargo run hello -s "Hello"` -> `Nothing found`
#### Case insensitive mode: `cargo run hello -s "Hello world"` -> `Hello` world

<hr/>

<br />

### Examples:

#### Case insensitive in file:
[<img src="./exaples/case_insensitive_in_file.png" width="600px"/>](https://github.com/b1rd-dev/rust_grep/blob/main/exaples/case_insensitive_in_file.png)

#### Case sensitive in string:
[<img src="./exaples/case_sensitive_in_string.png" width="600px"/>](https://github.com/b1rd-dev/rust_grep/blob/main/exaples/case_sensitive_in_string.png)

#### Case insensitive in string:

[<img src="./exaples/case_insensitive_in_string.png" width="600px"/>](https://github.com/b1rd-dev/rust_grep/blob/main/exaples/case_insensitive_in_string.png)
