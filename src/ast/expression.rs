#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Expr {
    Number(i32),
    Identifier(String),
    Op(Box<Expr>, Opcode, Box<Expr>),
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}
