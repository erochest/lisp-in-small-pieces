
pub struct Parser<'a, T> {
    input: Box<dyn Iterator<Item = T> + 'a>,
    buffer: Vec<T>,
}

impl<'a, T> Parser<'a, T> {
    fn new<I: Iterator<Item = T> + 'a>(input: I) -> Self {
        Parser { input: Box::new(input), buffer: Vec::new() }
    }

    fn shift(&mut self) {
        if let Some(token) = self.input.next() {
            self.buffer.push(token);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;

    #[derive(Debug, PartialEq)]
    enum Calculator {
        Number(isize),
        Plus,
        Minus,
        Multiply,
        Divide,
        // Not sure about having these in with the low-level token enum.
        Expr,
        Term,
    }

    use pretty_assertions::assert_eq;

    use Calculator::*;

    #[test]
    fn test_shift_adds_item() {
        let input = vec![Number(13), Plus, Number(42)];
        let input = input.iter();
        let mut parser = Parser::new(input);

        parser.shift();
        assert_eq!(parser.buffer.len(), 1);
        assert_eq!(*parser.buffer[0], Number(13));
    }
}