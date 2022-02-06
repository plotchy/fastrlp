#[derive(Clone, Default, PartialEq)]
pub struct Header {
    pub list: bool,
    pub payload_length: usize,
}
