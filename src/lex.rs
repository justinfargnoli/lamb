enum Token {
    ParenLeft,
    ParenRight,
    Comma,
    ID(String),
    Number(u32),
    TTrueC,
    TFalseC,
    TNumC,
    TPlusC,
    TMultC,
    TIfC,
    TIdC,
    TAppC,
    TFdC,
}

impl Iterator for Token {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        unimplemented!()
    }
}
