use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub enum AppEnvirontment {
    DEV,
    STAGING,
    PRODUCTION,
    TEST,
    PENTEST,
}

impl AppEnvirontment {
    pub fn value(&self) -> &'static str {
        match *self {
            Self::DEV => "devlopment",
            Self::PENTEST => "penetration_test",
            Self::PRODUCTION => "production",
            Self::STAGING => "staging",
            Self::TEST => "test",
        }
    }
}
