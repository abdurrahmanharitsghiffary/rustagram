pub enum ConsumerTag {
    EmailConsumer,
}

impl ConsumerTag {
    pub fn value(&self) -> &'static str {
        match *self {
            Self::EmailConsumer => "email_consumer",
        }
    }
}
