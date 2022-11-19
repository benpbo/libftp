pub struct Reply<'a> {
    pub code: &'a [u8],
    pub text: Text<'a>,
}

pub enum Text<'a> {
    SingleLine {
        line: &'a [u8],
    },
    MultiLine {
        lines: Vec<&'a [u8]>,
        last_line: &'a [u8],
    },
}
