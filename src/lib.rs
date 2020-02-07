mod lex;
mod parse;
mod read;
mod type_check;

// trait Input<T: Eq> {
//     fn peek(&self) -> Option<T>;
//     fn equals(&self, other: T) -> Option<T> {
//         if let self.peek() = other {
//             Option::Some(self.next())
//         }
//         Option::None
//     }
//     fn next(&self) -> Option<T>;
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
