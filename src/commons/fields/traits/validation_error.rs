pub trait ValidationError {
    fn description(&self) -> &'static str;
}

