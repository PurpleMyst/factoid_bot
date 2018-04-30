use super::{irc, simple_redis};

pub struct FactoidBot {
    irc_client: (),
    redis_client: (),
}

impl FactoidBot {
    pub fn new() -> Self {
        unimplemented!("FactoidBot::new");
    }

    fn get_factoid(&self, factoid: &str) -> Result<(), ()> {
        unimplemented!("FactoidBot::get_factoid");
    }

    fn set_factoid(&mut self, factoid: &str, response: &str) -> Result<(), ()> {
        unimplemented!("FactoidBot::set_factoid");
    }

    pub fn mainloop(&mut self) {
        unimplemented!("FactoidBot::mainloop");
    }
}
