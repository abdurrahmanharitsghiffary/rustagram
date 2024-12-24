pub enum QueueName {
    EmailQueue,
}

impl QueueName {
    pub fn value(&self) -> &'static str {
        match *self {
            Self::EmailQueue => "email_queue",
        }
    }
}
