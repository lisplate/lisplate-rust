#[derive(PartialEq, Clone, Debug)]
pub enum Literal {
  Float(f64),
  Int(i64),
  Bool(bool),
  Str(String),
}

#[derive(PartialEq, Clone, Debug)]
pub enum EscapeSeqs {
    RightBrace,
    LeftBrace,
    Space,
    NewLine,
    CarriageReturn
}

#[derive(PartialEq, Clone, Debug)]
pub enum Ast {
  Block(Vec<Box<Ast>>),
  Literal(Literal),
  Identifier(Option<String>, Option<String>),
  Format(String),
  Buffer(String),
  Escape(EscapeSeqs),
  Raw(String),
  FnCreate(Option<Vec<String>>, Box<Ast>), // TODO: can the 2nd, AST, be limited to block?
  Pipe(Box<Ast>, Vec<Box<Ast>>),
  Call(Box<Ast>, Vec<Box<Ast>>),

  Map(Option<Vec<(String, Box<Ast>)>>),
  Array(Option<Vec<Box<Ast>>>),
  Empty
}
