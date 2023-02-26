
pub struct Parser<'a, T> {
    input: Box<dyn Iterator<Item = T> + 'a>,
    buffer: Vec<T>,
}

impl<'a, T> Parser<'a, T>
where T: Parseable
{
    fn new<I: Iterator<Item = T> + 'a>(input: I) -> Self {
        Parser { input: Box::new(input), buffer: Vec::new() }
    }

    fn shift(&mut self) {
        if let Some(token) = self.input.next() {
            self.buffer.push(token);
        }
    }

    fn reduce(&mut self) -> bool {
        if let Some((to_replace, production)) = T::propose_reduction(&self.buffer) {
            self.buffer.truncate(self.buffer.len() - to_replace);
            self.buffer.push(production);
            true
        } else {
            false
        }
    }
}

trait Parseable {
    fn propose_reduction(buffer: &Vec<Self>) -> Option<(usize, Self)> where Self: Sized;
}

#[cfg(test)]
mod tests {
    use super::{Parser, Parseable};

    use pretty_assertions::assert_eq;

    #[derive(Debug, PartialEq, Clone)]
    enum Calculator {
        Number(isize),
        Plus,
        Minus,
        Multiply,
        Divide,

        // Not sure about having these in with the low-level token enum.
        // For one, we could get better type support

        // E -> T [+-] T
        Expr(Box<Calculator>, Box<Calculator>, Box<Calculator>),
        // T -> F [*/] F
        Term(Box<Calculator>, Box<Calculator>, Box<Calculator>),
        // F -> I
        // F -> -F
        // F -> ( E )
        Factor(Box<Calculator>),
    }

    impl Calculator {
        fn is_number(&self) -> bool {
            match self {
                Number(_) => true,
                _ => false,
            }
        }

        fn is_factor(&self) -> bool {
            match self {
                Factor(_) => true,
                _ => false,
            }
        }

        fn is_term_op(&self) -> bool {
            match self {
                Multiply|Divide => true,
                _ => false,
            }
        }

        fn is_expr_op(&self) -> bool {
            match self {
                Plus|Minus => true,
                _ => false,
            }
        }
    }

    impl Parseable for Calculator {
        fn propose_reduction(buffer: &Vec<Self>) -> Option<(usize, Self)> where Self: Sized {
            if let Some(last) = buffer.last() {
                if last.is_number() {
                    return Some((1, Factor(Box::new(last.clone()))))
                }
            }
            if let Some(end) = buffer.get(buffer.len()-3..buffer.len()) {
                if end[0].is_factor() && end[1].is_term_op() && end[2].is_factor() {
                    let f1 = end[0].clone();
                    let op = end[1].clone();
                    let f2 = end[2].clone();
                    return Some((3, Term(Box::new(f1), Box::new(op), Box::new(f2))))
                }
            }
            None
        }
    }

    use Calculator::*;

    #[test]
    fn test_shift_adds_item() {
        let input = vec![Number(13), Plus, Number(42)];
        let input = input.into_iter();
        let mut parser = Parser::new(input);

        parser.shift();
        assert_eq!(parser.buffer.len(), 1);
        assert_eq!(parser.buffer[0], Number(13));
    }

    #[test]
    fn test_shift_empty_does_nothing() {
        let input: Vec<Calculator> = vec![];
        let input = input.into_iter();
        let mut parser = Parser::new(input);

        parser.shift();
        assert!(parser.buffer.is_empty());
    }

    #[test]
    fn test_reduce_replaces_single_things() {
        let input = vec![Number(13), Plus, Number(42)];
        let input = input.into_iter();
        let mut parser = Parser::new(input);

        parser.shift();
        assert!(parser.reduce());
        assert_eq!(parser.buffer.len(), 1);
        assert_eq!(parser.buffer[0], Factor(Box::new(Number(13))))
    }

    #[test]
    fn test_reduce_replaces_lists() {
        let input = vec![Number(13), Multiply, Number(42)];
        let input = input.into_iter();
        let mut parser = Parser::new(input);

        parser.shift();
        assert!(parser.reduce());
        parser.shift();
        parser.shift();
        assert!(parser.reduce());
        assert!(parser.reduce());
        assert_eq!(parser.buffer.len(), 1);
        assert_eq!(parser.buffer[0], Term(
            Box::new(Factor(Box::new(Number(13)))),
            Box::new(Multiply),
            Box::new(Factor(Box::new(Number(42)))),
        ));
    }
}