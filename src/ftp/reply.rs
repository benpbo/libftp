#[derive(Debug)]
pub struct Reply {
    pub code: [u8; 3],
    pub text: Text,
}

#[derive(Debug)]
pub enum Text {
    SingleLine {
        line: Vec<u8>,
    },
    MultiLine {
        lines: Vec<Vec<u8>>,
        last_line: Vec<u8>,
    },
}
