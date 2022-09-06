#[derive(Debug)]
pub struct Value {
    pub value: String,
}

#[derive(Debug)]
pub struct Expression {
    pub value: Value,
}

#[derive(Debug)]
pub struct FromClause {
    pub value: Value,
}

#[derive(Debug)]
pub struct SelectClause {
    pub exps: Vec<Expression>,
}

#[derive(Debug)]
pub struct SelectStatement {
    pub select: SelectClause,
    pub from: Option<FromClause>,
}

#[derive(Debug)]
pub struct Ast {
    pub stmt: SelectStatement,
}
