# Wikit

Simple CLI program to get the summary of a wikipedia article.

Usage:
```Bash
$ wikit search-term
```

## Install

You can download the binary directly from [the release](../../releases) or [build it yourself](#to-build-yourself--)!

### To build yourself&#8212;
Make sure you have [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) to build the bin!

```Bash
$ git clone git@github.com:Ablexxive/wikit.git
$ cd wikit
$ cargo build --release
$ ln -s $PWD/target/release/wikit /usr/local/bin
```

And you should be good to go!
