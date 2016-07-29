#![feature(plugin)]
#![plugin(peg_syntax_ext)]

mod ast;
use ast::Literal;
use ast::EscapeSeqs;
use ast::Ast;

peg_file! lisplate("lisplate.rustpeg");

fn main() {
    let ret = lisplate::block("{{fn (one arr obj)
        {one}
        {+ {get arr 2} {get obj \"pi\"}}
    } p1 (\"p2\" 42) (:pi 3.14 :b true)}");

    println!("{:?}", ret);
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//     }
// }
