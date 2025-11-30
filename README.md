## gila-performative-banget-guweh

### Overview

Can also be mentioned as GPBG, is simple program that can make your literacy activity as a flex via discord RPC. You can understand why I made this do you?

This is also a some kind of training field for me. Cuz I am new to Rust and I wanted to figuring out how to walk with it. So I am very sorry if there is so much inefficiency, bugs, or flaws.

It will also fetches Google Books API to get the details of book that user is currently reading. So you guys does not have to input the details of the book (thanks to me).

### Installation

#### 1. Clone this repo

```
git clone https://github.com/jirbthagoras/gila-performative-banget-guweh.git
cd ./gila-performative-banget-guweh
```

#### 2. Build this project

```
cargo build --release
```

#### 3. Make it globally executable

```
sudo mv target/release/rpc /usr/local/bin/gpbg
sudo chmod +x /usr/local/bin/gpbg
```

#### 4. You're ready to go!

```
gpbg
```
