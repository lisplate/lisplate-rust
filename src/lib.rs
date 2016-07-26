#![feature(plugin)]
#![plugin(peg_syntax_ext)]

mod ast;
use ast::Literal;
use ast::EscapeSeqs;
use ast::Ast;

peg_file! modname("lisplate.rustpeg");

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
