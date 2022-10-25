#[derive(Debug)]
pub enum WordError {
    TooSmall(String),
}

#[derive(Debug)]
pub enum RequestedLengthError {
    TooSmall(String),
    TooBig(String),
}
