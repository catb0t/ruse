//! The type generated by the parser. It can then be evaluated by the
//! evaluator.

/// A Ruse value, which at the moment consists only of lists, strings,
/// integers, and floats.
#[derive(Debug)]
pub enum Expr {
    /// An atom
    Atom(String),
    /// A list
    List(Vec<Expr>),
    /// A dotted list
    DottedList(Vec<Expr>, Expr),
    /// An integer
    Integer(i64),
    /// A float
    Float(f64),
    /// A string
    Str(String),
    /// A bool
    Bool(bool),
}
