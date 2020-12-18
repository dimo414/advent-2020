use anyhow::{Context, Result};

pub fn advent() {
    let tokens = parse_data().unwrap();
    println!("Left-to-Right: {}",
             tokens.iter().map(|t| t.simple_expression().unwrap().evaluate()).sum::<i64>());
    println!("Addition first: {}",
             tokens.iter().map(|t| t.ordered_expression().unwrap().evaluate()).sum::<i64>());
}

struct Tokens<'a>(Vec<&'a str>);

impl<'a> Tokens<'a> {
    // Can't impl FromStr: https://stackoverflow.com/q/28931515/113632
    fn parse(str: &'a str) -> Result<Self> {
        let mut ret = Vec::new();
        let mut num_start = None;
        for i in 0..str.len() {
            let c = str.chars().skip(i).next().unwrap();
            match c {
                '0'..='9' => {
                    if num_start.is_none() { num_start = Some(i); }
                    if i == str.len()-1 || !str.chars().skip(i+1).next().unwrap().is_ascii_digit() {
                        let start = num_start.take().expect("Must be set");
                        ret.push(&str[start..i+1]);
                    }
                },
                '*'|'+'|'('|')' => {
                    ret.push(&str[i..i+1]);
                },
                ' ' => {},
                _ => anyhow::bail!("Unexpected character at {}: '{}'", i, c),
            }
        }
        Ok(Tokens(ret))
    }

    fn simple_expression(&self) -> Result<Expression> {
        use crate::aoc18::Expression::*;

        // Constructs an expression from the fewest possible tokens
        fn next_value<'a>(tokens: &'a[&'a str]) -> Result<(Expression, &'a[&'a str])> {
            if tokens[0] == "(" {
                let (op, remainder) = partial_parse(&tokens[1..])?;
                anyhow::ensure!(!remainder.is_empty());
                anyhow::ensure!(remainder[0] == ")", "Unexpected remainder {:?} after parsing {:?}", remainder, op);
                Ok((op, &remainder[1..]))
            } else {
                let literal = Literal(tokens[0].parse().with_context(|| format!("Unexpected token: {}", tokens[0]))?);
                Ok((literal, &tokens[1..]))
            }
        }

        // Constructs an expression from as many tokens as possible, halting at an unmatched ')'
        fn partial_parse<'a>(tokens: &'a[&'a str]) -> Result<(Expression, &'a[&'a str])> {
            anyhow::ensure!(!tokens.is_empty());
            let (mut expr, mut tokens) = next_value(tokens)?;
            while !tokens.is_empty() && tokens[0] != ")" {
                let op = tokens[0];
                let (right, remainder) = next_value(&tokens[1..])?;
                expr = match op {
                    "+" => Addition(Box::new(expr), Box::new(right)),
                    "*" => Multiplication(Box::new(expr), Box::new(right)),
                    _ => anyhow::bail!("Unexpected token: {}, following expr: {:?}", op, expr),
                };
                tokens = remainder;
            }
            Ok((expr, tokens))
        }

        let (expr, remainder) = partial_parse(&self.0)?;
        anyhow::ensure!(remainder.is_empty(), "Unparsed prefix: {:?} - parsed: {:?}", remainder, expr);
        Ok(expr)
    }

    fn ordered_expression(&self) -> Result<Expression> {
        use crate::aoc18::Expression::*;

        // Constructs an expression from the fewest possible tokens
        // This is identical to simple_expression(), it'd be nice to refactor so that we don't
        // redefine it, but notice this calls a different partial_parse() function.
        fn next_value<'a>(tokens: &'a[&'a str]) -> Result<(Expression, &'a[&'a str])> {
            if tokens[0] == "(" {
                let (op, remainder) = partial_parse(&tokens[1..])?;
                anyhow::ensure!(!remainder.is_empty());
                anyhow::ensure!(remainder[0] == ")", "Unexpected remainder {:?} after parsing {:?}", remainder, op);
                Ok((op, &remainder[1..]))
            } else {
                let literal = Literal(tokens[0].parse().with_context(|| format!("Unexpected token: {}", tokens[0]))?);
                Ok((literal, &tokens[1..]))
            }
        }

        // Constructs an expression from as many tokens as possible, halting at an unmatched ')'
        fn partial_parse<'a>(tokens: &'a[&'a str]) -> Result<(Expression, &'a[&'a str])> {
            anyhow::ensure!(!tokens.is_empty());
            let (mut expr, mut tokens) = next_value(tokens)?;
            while !tokens.is_empty() && tokens[0] != ")" {
                let op = tokens[0];
                expr = match op {
                    "+" => {
                        let (right, remainder) = next_value(&tokens[1..])?;
                        tokens = remainder;
                        Addition(Box::new(expr), Box::new(right))
                    },
                    "*" => {
                        let (right, remainder) = partial_parse(&tokens[1..])?;
                        tokens = remainder;
                        Multiplication(Box::new(expr), Box::new(right))
                    },
                    _ => anyhow::bail!("Unexpected token: {}, following expr: {:?}", op, expr),
                };
            }
            Ok((expr, tokens))
        }

        let (expr, remainder) = partial_parse(&self.0)?;
        anyhow::ensure!(remainder.is_empty(), "Unparsed prefix: {:?} - parsed: {:?}", remainder, expr);
        Ok(expr)
    }
}

#[derive(Debug)]
enum Expression {
    Literal(i64),
    Addition(Box<Expression>, Box<Expression>),
    Multiplication(Box<Expression>, Box<Expression>),
}

impl Expression {
    fn evaluate(&self) -> i64 {
        use crate::aoc18::Expression::*;
        match self {
            Literal(n) => *n,
            Addition(l, r) => l.evaluate() + r.evaluate(),
            Multiplication(l, r) => l.evaluate() * r.evaluate(),
        }
    }
}

fn parse_data() -> Result<Vec<Tokens<'static>>> {
    include_str!("../data/day18.txt").trim().split("\n").map(|e| Tokens::parse(e)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    parameterized_test::create!{simple, (expression, expected), {
        let tokens = Tokens::parse(expression).unwrap();
        assert_eq!(tokens.simple_expression().unwrap().evaluate(), expected);
    }}
    simple!{
      a: ("1 + 2 * 3 + 4 * 5 + 6", 71),
      b: ("1 + (2 * 3) + (4 * (5 + 6))", 51),
      c: ("2 * 3 + (4 * 5)", 26),
      d: ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437),
      e: ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240),
      f: ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632),
    }

    parameterized_test::create!{ordered, (expression, expected), {
        let tokens = Tokens::parse(expression).unwrap();
        assert_eq!(tokens.ordered_expression().unwrap().evaluate(), expected);
    }}
    ordered!{
      a: ("1 + 2 * 3 + 4 * 5 + 6", 231),
      b: ("1 + (2 * 3) + (4 * (5 + 6))", 51),
      c: ("2 * 3 + (4 * 5)", 46),
      d: ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445),
      e: ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060),
      f: ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340),
    }

    #[test]
    fn parse_file() {
        parse_data().unwrap();
    }
}
