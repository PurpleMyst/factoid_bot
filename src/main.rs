extern crate irc;

extern crate simple_redis;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate failure;

mod bot;

use irc::client::prelude::*;

fn main() {
    env_logger::init();

    let cfg = Config::load("config.toml").unwrap();
    let mut bot = bot::FactoidBot::new(cfg).unwrap();
    bot.mainloop().unwrap();
}
