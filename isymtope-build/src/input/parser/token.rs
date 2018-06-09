#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    UseKeyword,
    LetKeyword,
    ForKeyword,
    InKeyword,
    BindKeyword,
    AsKeyword,
    WhereKeyword,
    ToKeyword,

    QueryKeyword,

    SetKeyword,
    DeleteKeyword,
    UniqueKeyword,
    AndKeyword,

    ClientKeyword,
    ComponentKeyword,
    RouteKeyword,
    StoreKeyword,
    ActionKeyword,
    ExternKeyword,
    ModKeyword,
    ApiKeyword,
    ResourceKeyword,
    MethodsKeyword,

    GetKeyword,
    PostKeyword,
    PutKeyword,
    DelKeyword,
    PatchKeyword,

    EventKeyword,
    DispatchKeyword,
    NavigateKeyword,

    StateKeyword,
    ValueKeyword,
    ItemKeyword,

    MapKeyword,
    AutoKeyword,

    HashRocket,
    EqualTo,
    NotEqualTo,
    LessThan,
    GreaterThan,
    LessThanOrEqualTo,
    GreaterThanOrEqualTo,

    Pipe,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    OpenParen,
    CloseParen,
    Dot,
    Comma,
    Equals,
    Colon,
    Semi,
    Bang,
    Plus,
    Minus,
    Mul,
    Div,
    Identifier(String),
    LiteralNumber(i32),
    LiteralString(String),
    LiteralBool(bool),
    VariableReference(String),
}
