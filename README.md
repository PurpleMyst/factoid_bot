factoid_bot
===========

A simple factoid bot written in Rust + Redis.

Requirements
============

1. [Rust](https://www.rust-lang.org/en-US/install.html)
2. [Redis](https://redis.io/topics/quickstart)

Compilation
===========

Just `cargo build` will get you up to speed.

Running
=======

Before doing anything here, you may want to modify the `config.toml`. It's
pretty self-explanatory.

First off, start a redis server on `redis://localhost:6379`. The easiest way to
do this is to run `redis-server`, however there are more ways to do so that are
listed in the above-linked Redis quick start page.

Second, run `RUST_LOG=factoid_bot=info cargo run` to get the bot running. You
will now get a nice list of colorful output messages.
