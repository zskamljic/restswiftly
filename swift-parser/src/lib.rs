mod parsing;
#[cfg(test)]
mod test;
mod tokenizing;

#[derive(PartialEq, Debug)]
enum Token {
    LineComment(String),
    Identifier(String),
    LeftBrace,
    RightBrace,
    LeftParenthesis,
    RightParenthesis,
}
