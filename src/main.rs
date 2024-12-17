use crate::check::Error;

use check::type_and_fn_call_check;
use lexer::Lexeme;

mod lexer;
mod tree;
mod check;

fn main() -> Result<(), Error> {
    let code = "
    fn main(string: String) {
        let array = [0, 1, 2];

        if true { test } else {};
    }

    struct X {
        value: Type,
        another_value: String,
    }

    enum Option {
        Some(String),
        None,
    }

    main(String)
    ";

    let bytes = code.as_bytes().to_vec();

    let lexed = Lexeme::from_literal(bytes);
    // println!("{:?}", lexed);
    let tree = tree::tree(lexed);

    println!("AST: {:#?}", tree);

    type_and_fn_call_check(tree)?;

    Ok(())
}
