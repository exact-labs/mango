<p align="center"><img style="width: 300px;" src="https://cdn.justjs.dev/assets/svg/mango_title.svg" /></p>

##

Mango is a simple key value database meant for your cli. Helps you store values for your scripts.

### Quick Start

See the [installation](#installation) section for how to install just on your computer. Try running `mango --version` to make sure that it's installed correctly.

```bash
~ $ mango set hello world
added key 'hello' with value 'world'
```

Mango supports setting/removing values without any output, with the `-q` or `--quiet` argument

```bash
~ $ mango del hello -q
# notice the lack of output
```

Mango also supports saving the entire store to a file

```bash
~ $ mango save file.txt
saved store '~/.mango' to 'file.txt'
```

For more commands, check out `mango --help`

### Installation

Pre-built binaries for Linux, MacOS, and Windows can be found on the [releases](https://github.com/exact-labs/mango/releases) page.

#### Building

- Clone the project `git clone https://github.com/exact-labs/mango.git`
- Open a terminal in the project folder
- Check if you have cargo (Rust's package manager) installed, just type in `cargo`
- If cargo is installed, run `cargo build --release`
- Put the executable into one of your PATH entries
  - Linux: usually /bin/ or /usr/bin/
  - Windows: C:\Windows\System32 is good for it but don't use windows
