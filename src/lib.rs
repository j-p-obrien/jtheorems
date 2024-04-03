type Level = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Universe {
    name: &'static str,
    level: Level,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Type {
    name: &'static str,
    universe: Universe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Variable {
    name: &'static str,
    dtype: Type,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Context(Vec<Variable>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Conclusion {
    Context(Context),
    Term(Variable),
    Equal(Variable, Variable),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Judgement {
    context: Context,
    conlusion: Conclusion,
}
