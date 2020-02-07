mod lex;
mod parse;
mod read;
mod type_check;

trait Input<T: Eq> {
    fn peek(&self) -> T;
    fn equals(&self, other: T) -> Option<T> {
        if self.peek() == other {
            Option::Some(self.next())
        } else {
            Option::None
        }
    }
    fn next(&self) -> T;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
