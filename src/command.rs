use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Command {
    Publish(String, String),
    Subscribe(String),
    Unsubscribe(String),
    PatternSubscribe(String),
    PatternUnsubscribe(String),
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let d = match self {
            Command::Publish(chan, msg) => {
                f.write_str(format!("PUBLISH {} \"{}\"", chan, msg).as_str())?;
                chan
            }
            Command::Subscribe(t) => {
                f.write_str("SUBSCRIBE ")?;
                t
            }
            Command::Unsubscribe(t) => {
                f.write_str("UNSUBSCRIBE ")?;
                t
            }
            Command::PatternSubscribe(t) => {
                f.write_str("PSUBSCRIBE ")?;
                t
            }
            Command::PatternUnsubscribe(t) => {
                f.write_str("PUNSUBSCRIBE ")?;
                t
            }
        };
        f.write_str(d)?;
        f.write_str("\r\n")
    }
}
