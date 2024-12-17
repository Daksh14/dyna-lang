// this module does the type checking and checking if function exists
// this runs on a syntax tree representation

use crate::tree::{Ast, DynaFunction, DynaType};

#[derive(Debug)]
pub enum Error {
    FunctionDoesntExist,
    TypeMistach,
}

pub fn type_and_fn_call_check(tree: Vec<Ast>) -> Result<(), Error> {
    for tree_item in tree.clone() {
        if let Ast::FunctionCall(call) = tree_item {
            let name_of_function_being_called = call.fn_name;
            let lookup = tree_lookup_function(tree.clone(), name_of_function_being_called)?;

            let arguments: Vec<DynaType> = lookup.signature.values().cloned().collect();

            if call.signature != arguments {
                return Err(Error::TypeMistach)
            }
        }
    }

    Ok(())
}

fn tree_lookup_function(tree: Vec<Ast>, fn_name: String) -> Result<DynaFunction, Error> {
    for tree_item in tree {
        if let Ast::Function(function) = tree_item {
            if function.name == fn_name {
                return Ok(function);
            }
        }
    }

    Err(Error::FunctionDoesntExist)
}
