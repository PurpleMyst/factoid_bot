use failure::Error;
use irc::{client::prelude::*, error::IrcError};
use simple_redis;

pub struct FactoidBot {
    irc_client: IrcClient,
    redis_client: simple_redis::client::Client,
}

impl FactoidBot {
    pub fn new(cfg: Config) -> Result<Self, Error> {
        Ok(Self {
            irc_client: IrcClient::from_config(cfg)?,
            redis_client: simple_redis::create("redis://localhost:6379")?,
        })
    }

    fn get_factoid(&mut self, factoid: &str) -> Result<String, Error> {
        Ok(self.redis_client
            .get_string(&format!("factoid:{}", factoid))?)
    }

    fn set_factoid(&mut self, factoid: &str, response: &str) -> Result<(), Error> {
        self.redis_client
            .set(&format!("factoid:{}", factoid), response)?;
        Ok(())
    }

    fn get_response(&mut self, cmd: &str, args: &[&str]) -> Result<String, Error> {
        info!("Responding to command ${} {}", cmd, args.join(" "));

        match cmd {
            "at" if args.len() >= 2 => {
                let target = args[0];
                let factoid = args[1];

                Ok(format!("{}: {}", target, self.get_factoid(factoid)?))
            }

            "defact" if args.len() > 1 => {
                let factoid = args[0];
                let response = args[1..].join(" ");
                self.set_factoid(factoid, &response)?;
                Ok(format!("Defined factoid '{}'", factoid))
            }

            "factoids" => {
                let prefix_len = "factoid:".len();
                let factoid_names = self.redis_client
                    .keys("factoid:*")?
                    .iter()
                    .map(|s| &s[prefix_len..])
                    .collect::<Vec<&str>>()
                    .join(" ");
                Ok(factoid_names)
            }

            "join" if args.len() >= 1 => {
                let joined_channels = args.iter()
                    .cloned()
                    .filter(|channel| {
                        // XXX: Should we store these channels in the redis database?
                        let join_result = self.irc_client.send_join(channel);

                        if let Err(ref err) = join_result {
                            info!("Failed to join {}: {}", channel, err);
                        } else {
                            info!("Successfully joined {}", channel);
                        }

                        join_result.is_ok()
                    })
                    .collect::<Vec<&str>>()
                    .join(" ");

                Ok(format!("Successfully joined {}", joined_channels))
            }

            _ => self.get_factoid(cmd),
        }
    }

    fn handle_message(&mut self, msg: Message) -> Result<(), IrcError> {
        if let Command::PRIVMSG(ref _target, ref text) = msg.command {
            let mut parts = text.split_whitespace();

            if let Some(cmd) = parts.next() {
                if cmd.starts_with('$') {
                    let cmd = &cmd[1..];
                    let args = parts.collect::<Vec<&str>>();

                    match self.get_response(cmd, &args) {
                        Ok(response) => {
                            self.irc_client
                                .send_privmsg(msg.response_target().unwrap(), &response)?;
                        }

                        Err(err) => {
                            info!(
                                "Got no response for factoid ${} {} (reason: {})",
                                cmd,
                                args.join(" "),
                                err
                            );
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub fn mainloop(&mut self) -> Result<(), Error> {
        self.irc_client.identify()?;
        self.irc_client
            .stream()
            .for_each(|msg| self.handle_message(msg))
            .wait()?;
        Ok(())
    }
}
