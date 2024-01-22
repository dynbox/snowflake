use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct LimitExceeded;

impl Display for LimitExceeded {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Limit of entities exceeded error occurred")
    }
}

impl Error for LimitExceeded {}