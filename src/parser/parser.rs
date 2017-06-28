use parser::ast::{NodeType, ComponentDefinitionType, ElementType, ExprType, ExprValue};
use parser::token::Token;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Template {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use parser::ast::{NodeType, ComponentDefinitionType, ElementType, ExprType, ExprValue};
    use parser::token::Token;
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_28_22(Token<'input>),
        Term_22_29_22(Token<'input>),
        Term_22_2c_22(Token<'input>),
        Term_22_3d_22(Token<'input>),
        Term_22_7b_22(Token<'input>),
        Term_22_7d_22(Token<'input>),
        TermComponentKeyword(Token<'input>),
        TermId(&'input str),
        TermLitStr(String),
        Nt_22_2c_22_3f(::std::option::Option<Token<'input>>),
        Nt_28_3cParam_3e_20_22_2c_22_29((String, ExprValue)),
        Nt_28_3cParam_3e_20_22_2c_22_29_2a(::std::vec::Vec<(String, ExprValue)>),
        Nt_28_3cParam_3e_20_22_2c_22_29_2b(::std::vec::Vec<(String, ExprValue)>),
        Nt_28Id_20_22_2c_22_3f_29((&'input str, ::std::option::Option<Token<'input>>)),
        Nt_28Id_20_22_2c_22_3f_29_2a(::std::vec::Vec<(&'input str, ::std::option::Option<Token<'input>>)>),
        Nt_28Id_20_22_2c_22_3f_29_2b(::std::vec::Vec<(&'input str, ::std::option::Option<Token<'input>>)>),
        NtComponentDefinitionNode(NodeType),
        NtElementNode(NodeType),
        NtExprValue(ExprValue),
        NtExpressionNode(NodeType),
        NtNodeType(NodeType),
        NtNodeType_2a(::std::vec::Vec<NodeType>),
        NtNodeType_2b(::std::vec::Vec<NodeType>),
        NtParam((String, ExprValue)),
        NtParam_3f(::std::option::Option<(String, ExprValue)>),
        NtParamList(Vec<(String, ExprValue)>),
        NtTemplate(Vec<NodeType>),
        Nt____Template(Vec<NodeType>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 8, 9, 10,
        // State 1
        -40, -40, -40, -40, -40, -40, -40, -40, -40,
        // State 2
        -41, -41, -41, -41, -41, -41, -41, -41, -41,
        // State 3
        -42, -42, -42, -42, -42, -42, -42, -42, -42,
        // State 4
        -45, -45, -45, -45, -45, -45, -45, -45, -45,
        // State 5
        0, 0, 0, 0, 0, 0, 8, 9, 10,
        // State 6
        -56, -56, -56, -56, -56, -56, -56, -56, -56,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 12, 0,
        // State 8
        13, 0, 0, 0, 14, -38, -38, -38, -38,
        // State 9
        -39, -39, -39, -39, -39, -39, -39, -39, -39,
        // State 10
        -46, -46, -46, -46, -46, -46, -46, -46, -46,
        // State 11
        15, 0, 0, 0, 16, 0, 0, 0, 0,
        // State 12
        0, 18, 0, 0, 0, 0, 0, 19, 0,
        // State 13
        0, 0, 0, 0, 0, 21, 8, 9, 10,
        // State 14
        0, 23, 0, 0, 0, 0, 0, 24, 0,
        // State 15
        0, 0, 0, 0, 0, 26, 8, 9, 10,
        // State 16
        0, 27, 0, 0, 0, 0, 0, 28, 0,
        // State 17
        0, 0, 0, 0, 29, -33, -33, -33, -33,
        // State 18
        0, 0, 0, 30, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 31, 8, 9, 10,
        // State 20
        -22, -22, -22, -22, -22, -22, -22, -22, -22,
        // State 21
        0, 32, 0, 0, 0, 0, 0, 33, 0,
        // State 22
        0, 0, 0, 0, 34, 0, 0, 0, 0,
        // State 23
        0, -13, 35, 0, 0, 0, 0, -13, 0,
        // State 24
        0, 0, 0, 0, 0, 36, 8, 9, 10,
        // State 25
        -16, -16, -16, -16, -16, -16, -16, -16, -16,
        // State 26
        0, 0, 0, 0, 37, -35, -35, -35, -35,
        // State 27
        0, 0, 0, 38, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 40, 8, 9, 10,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 42, 43,
        // State 30
        -23, -23, -23, -23, -23, -23, -23, -23, -23,
        // State 31
        0, 0, 0, 0, 44, 0, 0, 0, 0,
        // State 32
        0, -15, 45, 0, 0, 0, 0, -15, 0,
        // State 33
        0, 0, 0, 0, 0, 47, 8, 9, 10,
        // State 34
        -12, -12, -12, -12, -12, -12, -12, -12, -12,
        // State 35
        -17, -17, -17, -17, -17, -17, -17, -17, -17,
        // State 36
        0, 0, 0, 0, 0, 49, 8, 9, 10,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 42, 43,
        // State 38
        0, 0, 0, 0, 0, 51, 8, 9, 10,
        // State 39
        -25, -25, -25, -25, -25, -25, -25, -25, -25,
        // State 40
        0, 52, 53, 0, 0, 0, 0, 0, 0,
        // State 41
        -36, -36, -36, -36, -36, -36, -36, -36, -36,
        // State 42
        -37, -37, -37, -37, -37, -37, -37, -37, -37,
        // State 43
        0, 0, 0, 0, 0, 55, 8, 9, 10,
        // State 44
        -14, -14, -14, -14, -14, -14, -14, -14, -14,
        // State 45
        0, 0, 0, 0, 0, 56, 8, 9, 10,
        // State 46
        -18, -18, -18, -18, -18, -18, -18, -18, -18,
        // State 47
        0, 0, 0, 0, 0, 57, 8, 9, 10,
        // State 48
        -27, -27, -27, -27, -27, -27, -27, -27, -27,
        // State 49
        0, 58, 59, 0, 0, 0, 0, 0, 0,
        // State 50
        -29, -29, -29, -29, -29, -29, -29, -29, -29,
        // State 51
        0, 0, 0, 0, 60, -32, -32, -32, -32,
        // State 52
        -6, -6, -6, -6, -6, -6, -6, -6, -6,
        // State 53
        0, 0, 0, 0, 0, 61, 8, 9, 10,
        // State 54
        -20, -20, -20, -20, -20, -20, -20, -20, -20,
        // State 55
        -19, -19, -19, -19, -19, -19, -19, -19, -19,
        // State 56
        -31, -31, -31, -31, -31, -31, -31, -31, -31,
        // State 57
        0, 0, 0, 0, 62, -34, -34, -34, -34,
        // State 58
        -7, -7, -7, -7, -7, -7, -7, -7, -7,
        // State 59
        0, 0, 0, 0, 0, 64, 8, 9, 10,
        // State 60
        -21, -21, -21, -21, -21, -21, -21, -21, -21,
        // State 61
        0, 0, 0, 0, 0, 66, 8, 9, 10,
        // State 62
        0, 0, 0, 0, 0, 67, 8, 9, 10,
        // State 63
        -24, -24, -24, -24, -24, -24, -24, -24, -24,
        // State 64
        0, 0, 0, 0, 0, 68, 8, 9, 10,
        // State 65
        -26, -26, -26, -26, -26, -26, -26, -26, -26,
        // State 66
        -28, -28, -28, -28, -28, -28, -28, -28, -28,
        // State 67
        -30, -30, -30, -30, -30, -30, -30, -30, -30,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        -54,
        -40,
        -41,
        -42,
        -45,
        -55,
        -56,
        0,
        -38,
        -39,
        -46,
        0,
        0,
        0,
        0,
        0,
        0,
        -33,
        0,
        0,
        -22,
        0,
        0,
        0,
        0,
        -16,
        -35,
        0,
        0,
        0,
        -23,
        0,
        0,
        0,
        -12,
        -17,
        0,
        0,
        0,
        -25,
        0,
        -36,
        -37,
        0,
        -14,
        0,
        -18,
        0,
        -27,
        0,
        -29,
        -32,
        -6,
        0,
        -20,
        -19,
        -31,
        -34,
        -7,
        0,
        -21,
        0,
        0,
        -24,
        0,
        -26,
        -28,
        -30,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 4, 5, 0, 6, 0, 0, 0, 7, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 4, 11, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 4, 5, 0, 20, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 4, 5, 0, 25, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 4, 11, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 4, 11, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 4, 5, 0, 39, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 4, 5, 0, 46, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 4, 5, 0, 48, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 4, 11, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 4, 5, 0, 54, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 4, 11, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 4, 11, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 4, 11, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 4, 5, 0, 63, 0, 0, 0, 0, 0,
        // State 60
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 61
        0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 4, 5, 0, 65, 0, 0, 0, 0, 0,
        // State 62
        0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 4, 11, 0, 0, 0, 0, 0, 0, 0,
        // State 63
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 4, 11, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 66
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###"",""###,
            r###""=""###,
            r###""{""###,
            r###""}""###,
            r###"ComponentKeyword"###,
            r###"Id"###,
            r###"LitStr"###,
        ];
        __ACTION[(__state * 9)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_Template<
        'input,
        __TOKEN: __ToTriple<'input, Error=()>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        __tokens0: __TOKENS,
    ) -> Result<Vec<NodeType>, __lalrpop_util::ParseError<(), Token<'input>, ()>>
    {
        let __tokens = __tokens0.into_iter();
        let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                Token::OpenParen if true => 0,
                Token::CloseParen if true => 1,
                Token::Comma if true => 2,
                Token::Equals if true => 3,
                Token::OpenBrace if true => 4,
                Token::CloseBrace if true => 5,
                Token::ComponentKeyword if true => 6,
                Token::Identifier(_) if true => 7,
                Token::LiteralString(_) if true => 8,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 9 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            __tok @ Token::OpenParen => __Symbol::Term_22_28_22((__tok)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            __tok @ Token::CloseParen => __Symbol::Term_22_29_22((__tok)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            __tok @ Token::Comma => __Symbol::Term_22_2c_22((__tok)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            __tok @ Token::Equals => __Symbol::Term_22_3d_22((__tok)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            __tok @ Token::OpenBrace => __Symbol::Term_22_7b_22((__tok)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            __tok @ Token::CloseBrace => __Symbol::Term_22_7d_22((__tok)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            __tok @ Token::ComponentKeyword => __Symbol::TermComponentKeyword((__tok)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            Token::Identifier(__tok0) => __Symbol::TermId((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            Token::LiteralString(__tok0) => __Symbol::TermLitStr((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        __action: i32,
        __lookahead_start: Option<&()>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<((),__Symbol<'input>,())>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Vec<NodeType>,__lalrpop_util::ParseError<(), Token<'input>, ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // ","? = "," => ActionFn(24);
                let __sym0 = __pop_Term_22_2c_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_22_2c_22_3f(__nt), __end));
                0
            }
            2 => {
                // ","? =  => ActionFn(25);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action25::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_22_2c_22_3f(__nt), __end));
                0
            }
            3 => {
                // (<Param> ",") = Id, "=", ExprValue, "," => ActionFn(36);
                let __sym3 = __pop_Term_22_2c_22(__symbols);
                let __sym2 = __pop_NtExprValue(__symbols);
                let __sym1 = __pop_Term_22_3d_22(__symbols);
                let __sym0 = __pop_TermId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action36::<>(__sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::Nt_28_3cParam_3e_20_22_2c_22_29(__nt), __end));
                1
            }
            4 => {
                // (<Param> ",")* =  => ActionFn(18);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action18::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cParam_3e_20_22_2c_22_29_2a(__nt), __end));
                2
            }
            5 => {
                // (<Param> ",")* = (<Param> ",")+ => ActionFn(19);
                let __sym0 = __pop_Nt_28_3cParam_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cParam_3e_20_22_2c_22_29_2a(__nt), __end));
                2
            }
            6 => {
                // (<Param> ",")+ = Id, "=", ExprValue, "," => ActionFn(38);
                let __sym3 = __pop_Term_22_2c_22(__symbols);
                let __sym2 = __pop_NtExprValue(__symbols);
                let __sym1 = __pop_Term_22_3d_22(__symbols);
                let __sym0 = __pop_TermId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action38::<>(__sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::Nt_28_3cParam_3e_20_22_2c_22_29_2b(__nt), __end));
                3
            }
            7 => {
                // (<Param> ",")+ = (<Param> ",")+, Id, "=", ExprValue, "," => ActionFn(39);
                let __sym4 = __pop_Term_22_2c_22(__symbols);
                let __sym3 = __pop_NtExprValue(__symbols);
                let __sym2 = __pop_Term_22_3d_22(__symbols);
                let __sym1 = __pop_TermId(__symbols);
                let __sym0 = __pop_Nt_28_3cParam_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action39::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::Nt_28_3cParam_3e_20_22_2c_22_29_2b(__nt), __end));
                3
            }
            8 => {
                // (Id ","?) = Id, "," => ActionFn(34);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_TermId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action34::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28Id_20_22_2c_22_3f_29(__nt), __end));
                4
            }
            9 => {
                // (Id ","?) = Id => ActionFn(35);
                let __sym0 = __pop_TermId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action35::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28Id_20_22_2c_22_3f_29(__nt), __end));
                4
            }
            10 => {
                // (Id ","?)* =  => ActionFn(21);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action21::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28Id_20_22_2c_22_3f_29_2a(__nt), __end));
                5
            }
            11 => {
                // (Id ","?)* = (Id ","?)+ => ActionFn(22);
                let __sym0 = __pop_Nt_28Id_20_22_2c_22_3f_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28Id_20_22_2c_22_3f_29_2a(__nt), __end));
                5
            }
            12 => {
                // (Id ","?)+ = Id, "," => ActionFn(42);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_TermId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action42::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28Id_20_22_2c_22_3f_29_2b(__nt), __end));
                6
            }
            13 => {
                // (Id ","?)+ = Id => ActionFn(43);
                let __sym0 = __pop_TermId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action43::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28Id_20_22_2c_22_3f_29_2b(__nt), __end));
                6
            }
            14 => {
                // (Id ","?)+ = (Id ","?)+, Id, "," => ActionFn(44);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_TermId(__symbols);
                let __sym0 = __pop_Nt_28Id_20_22_2c_22_3f_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action44::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28Id_20_22_2c_22_3f_29_2b(__nt), __end));
                6
            }
            15 => {
                // (Id ","?)+ = (Id ","?)+, Id => ActionFn(45);
                let __sym1 = __pop_TermId(__symbols);
                let __sym0 = __pop_Nt_28Id_20_22_2c_22_3f_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action45::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28Id_20_22_2c_22_3f_29_2b(__nt), __end));
                6
            }
            16 => {
                // ComponentDefinitionNode = ComponentKeyword, Id, "{", "}" => ActionFn(48);
                let __sym3 = __pop_Term_22_7d_22(__symbols);
                let __sym2 = __pop_Term_22_7b_22(__symbols);
                let __sym1 = __pop_TermId(__symbols);
                let __sym0 = __pop_TermComponentKeyword(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action48::<>(__sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtComponentDefinitionNode(__nt), __end));
                7
            }
            17 => {
                // ComponentDefinitionNode = ComponentKeyword, Id, "{", NodeType+, "}" => ActionFn(49);
                let __sym4 = __pop_Term_22_7d_22(__symbols);
                let __sym3 = __pop_NtNodeType_2b(__symbols);
                let __sym2 = __pop_Term_22_7b_22(__symbols);
                let __sym1 = __pop_TermId(__symbols);
                let __sym0 = __pop_TermComponentKeyword(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action49::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtComponentDefinitionNode(__nt), __end));
                7
            }
            18 => {
                // ComponentDefinitionNode = ComponentKeyword, Id, "(", ")", "{", "}" => ActionFn(50);
                let __sym5 = __pop_Term_22_7d_22(__symbols);
                let __sym4 = __pop_Term_22_7b_22(__symbols);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_Term_22_28_22(__symbols);
                let __sym1 = __pop_TermId(__symbols);
                let __sym0 = __pop_TermComponentKeyword(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action50::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtComponentDefinitionNode(__nt), __end));
                7
            }
            19 => {
                // ComponentDefinitionNode = ComponentKeyword, Id, "(", ")", "{", NodeType+, "}" => ActionFn(51);
                let __sym6 = __pop_Term_22_7d_22(__symbols);
                let __sym5 = __pop_NtNodeType_2b(__symbols);
                let __sym4 = __pop_Term_22_7b_22(__symbols);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_Term_22_28_22(__symbols);
                let __sym1 = __pop_TermId(__symbols);
                let __sym0 = __pop_TermComponentKeyword(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym6.2.clone();
                let __nt = super::__action51::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
                let __states_len = __states.len();
                __states.truncate(__states_len - 7);
                __symbols.push((__start, __Symbol::NtComponentDefinitionNode(__nt), __end));
                7
            }
            20 => {
                // ComponentDefinitionNode = ComponentKeyword, Id, "(", (Id ","?)+, ")", "{", "}" => ActionFn(52);
                let __sym6 = __pop_Term_22_7d_22(__symbols);
                let __sym5 = __pop_Term_22_7b_22(__symbols);
                let __sym4 = __pop_Term_22_29_22(__symbols);
                let __sym3 = __pop_Nt_28Id_20_22_2c_22_3f_29_2b(__symbols);
                let __sym2 = __pop_Term_22_28_22(__symbols);
                let __sym1 = __pop_TermId(__symbols);
                let __sym0 = __pop_TermComponentKeyword(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym6.2.clone();
                let __nt = super::__action52::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
                let __states_len = __states.len();
                __states.truncate(__states_len - 7);
                __symbols.push((__start, __Symbol::NtComponentDefinitionNode(__nt), __end));
                7
            }
            21 => {
                // ComponentDefinitionNode = ComponentKeyword, Id, "(", (Id ","?)+, ")", "{", NodeType+, "}" => ActionFn(53);
                let __sym7 = __pop_Term_22_7d_22(__symbols);
                let __sym6 = __pop_NtNodeType_2b(__symbols);
                let __sym5 = __pop_Term_22_7b_22(__symbols);
                let __sym4 = __pop_Term_22_29_22(__symbols);
                let __sym3 = __pop_Nt_28Id_20_22_2c_22_3f_29_2b(__symbols);
                let __sym2 = __pop_Term_22_28_22(__symbols);
                let __sym1 = __pop_TermId(__symbols);
                let __sym0 = __pop_TermComponentKeyword(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym7.2.clone();
                let __nt = super::__action53::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
                let __states_len = __states.len();
                __states.truncate(__states_len - 8);
                __symbols.push((__start, __Symbol::NtComponentDefinitionNode(__nt), __end));
                7
            }
            22 => {
                // ElementNode = Id, "{", "}" => ActionFn(54);
                let __sym2 = __pop_Term_22_7d_22(__symbols);
                let __sym1 = __pop_Term_22_7b_22(__symbols);
                let __sym0 = __pop_TermId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action54::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtElementNode(__nt), __end));
                8
            }
            23 => {
                // ElementNode = Id, "{", NodeType+, "}" => ActionFn(55);
                let __sym3 = __pop_Term_22_7d_22(__symbols);
                let __sym2 = __pop_NtNodeType_2b(__symbols);
                let __sym1 = __pop_Term_22_7b_22(__symbols);
                let __sym0 = __pop_TermId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action55::<>(__sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtElementNode(__nt), __end));
                8
            }
            24 => {
                // ElementNode = Id, "(", Id, "=", ExprValue, ")", "{", "}" => ActionFn(64);
                let __sym7 = __pop_Term_22_7d_22(__symbols);
                let __sym6 = __pop_Term_22_7b_22(__symbols);
                let __sym5 = __pop_Term_22_29_22(__symbols);
                let __sym4 = __pop_NtExprValue(__symbols);
                let __sym3 = __pop_Term_22_3d_22(__symbols);
                let __sym2 = __pop_TermId(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_TermId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym7.2.clone();
                let __nt = super::__action64::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
                let __states_len = __states.len();
                __states.truncate(__states_len - 8);
                __symbols.push((__start, __Symbol::NtElementNode(__nt), __end));
                8
            }
            25 => {
                // ElementNode = Id, "(", ")", "{", "}" => ActionFn(65);
                let __sym4 = __pop_Term_22_7d_22(__symbols);
                let __sym3 = __pop_Term_22_7b_22(__symbols);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_TermId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action65::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtElementNode(__nt), __end));
                8
            }
            26 => {
                // ElementNode = Id, "(", (<Param> ",")+, Id, "=", ExprValue, ")", "{", "}" => ActionFn(66);
                let __sym8 = __pop_Term_22_7d_22(__symbols);
                let __sym7 = __pop_Term_22_7b_22(__symbols);
                let __sym6 = __pop_Term_22_29_22(__symbols);
                let __sym5 = __pop_NtExprValue(__symbols);
                let __sym4 = __pop_Term_22_3d_22(__symbols);
                let __sym3 = __pop_TermId(__symbols);
                let __sym2 = __pop_Nt_28_3cParam_3e_20_22_2c_22_29_2b(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_TermId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym8.2.clone();
                let __nt = super::__action66::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7, __sym8);
                let __states_len = __states.len();
                __states.truncate(__states_len - 9);
                __symbols.push((__start, __Symbol::NtElementNode(__nt), __end));
                8
            }
            27 => {
                // ElementNode = Id, "(", (<Param> ",")+, ")", "{", "}" => ActionFn(67);
                let __sym5 = __pop_Term_22_7d_22(__symbols);
                let __sym4 = __pop_Term_22_7b_22(__symbols);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_Nt_28_3cParam_3e_20_22_2c_22_29_2b(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_TermId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action67::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtElementNode(__nt), __end));
                8
            }
            28 => {
                // ElementNode = Id, "(", Id, "=", ExprValue, ")", "{", NodeType+, "}" => ActionFn(68);
                let __sym8 = __pop_Term_22_7d_22(__symbols);
                let __sym7 = __pop_NtNodeType_2b(__symbols);
                let __sym6 = __pop_Term_22_7b_22(__symbols);
                let __sym5 = __pop_Term_22_29_22(__symbols);
                let __sym4 = __pop_NtExprValue(__symbols);
                let __sym3 = __pop_Term_22_3d_22(__symbols);
                let __sym2 = __pop_TermId(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_TermId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym8.2.clone();
                let __nt = super::__action68::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7, __sym8);
                let __states_len = __states.len();
                __states.truncate(__states_len - 9);
                __symbols.push((__start, __Symbol::NtElementNode(__nt), __end));
                8
            }
            29 => {
                // ElementNode = Id, "(", ")", "{", NodeType+, "}" => ActionFn(69);
                let __sym5 = __pop_Term_22_7d_22(__symbols);
                let __sym4 = __pop_NtNodeType_2b(__symbols);
                let __sym3 = __pop_Term_22_7b_22(__symbols);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_TermId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action69::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtElementNode(__nt), __end));
                8
            }
            30 => {
                // ElementNode = Id, "(", (<Param> ",")+, Id, "=", ExprValue, ")", "{", NodeType+, "}" => ActionFn(70);
                let __sym9 = __pop_Term_22_7d_22(__symbols);
                let __sym8 = __pop_NtNodeType_2b(__symbols);
                let __sym7 = __pop_Term_22_7b_22(__symbols);
                let __sym6 = __pop_Term_22_29_22(__symbols);
                let __sym5 = __pop_NtExprValue(__symbols);
                let __sym4 = __pop_Term_22_3d_22(__symbols);
                let __sym3 = __pop_TermId(__symbols);
                let __sym2 = __pop_Nt_28_3cParam_3e_20_22_2c_22_29_2b(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_TermId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym9.2.clone();
                let __nt = super::__action70::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7, __sym8, __sym9);
                let __states_len = __states.len();
                __states.truncate(__states_len - 10);
                __symbols.push((__start, __Symbol::NtElementNode(__nt), __end));
                8
            }
            31 => {
                // ElementNode = Id, "(", (<Param> ",")+, ")", "{", NodeType+, "}" => ActionFn(71);
                let __sym6 = __pop_Term_22_7d_22(__symbols);
                let __sym5 = __pop_NtNodeType_2b(__symbols);
                let __sym4 = __pop_Term_22_7b_22(__symbols);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_Nt_28_3cParam_3e_20_22_2c_22_29_2b(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_TermId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym6.2.clone();
                let __nt = super::__action71::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
                let __states_len = __states.len();
                __states.truncate(__states_len - 7);
                __symbols.push((__start, __Symbol::NtElementNode(__nt), __end));
                8
            }
            32 => {
                // ElementNode = Id, "(", Id, "=", ExprValue, ")" => ActionFn(72);
                let __sym5 = __pop_Term_22_29_22(__symbols);
                let __sym4 = __pop_NtExprValue(__symbols);
                let __sym3 = __pop_Term_22_3d_22(__symbols);
                let __sym2 = __pop_TermId(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_TermId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action72::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtElementNode(__nt), __end));
                8
            }
            33 => {
                // ElementNode = Id, "(", ")" => ActionFn(73);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_TermId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action73::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtElementNode(__nt), __end));
                8
            }
            34 => {
                // ElementNode = Id, "(", (<Param> ",")+, Id, "=", ExprValue, ")" => ActionFn(74);
                let __sym6 = __pop_Term_22_29_22(__symbols);
                let __sym5 = __pop_NtExprValue(__symbols);
                let __sym4 = __pop_Term_22_3d_22(__symbols);
                let __sym3 = __pop_TermId(__symbols);
                let __sym2 = __pop_Nt_28_3cParam_3e_20_22_2c_22_29_2b(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_TermId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym6.2.clone();
                let __nt = super::__action74::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
                let __states_len = __states.len();
                __states.truncate(__states_len - 7);
                __symbols.push((__start, __Symbol::NtElementNode(__nt), __end));
                8
            }
            35 => {
                // ElementNode = Id, "(", (<Param> ",")+, ")" => ActionFn(75);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_Nt_28_3cParam_3e_20_22_2c_22_29_2b(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_TermId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action75::<>(__sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtElementNode(__nt), __end));
                8
            }
            36 => {
                // ExprValue = Id => ActionFn(7);
                let __sym0 = __pop_TermId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExprValue(__nt), __end));
                9
            }
            37 => {
                // ExprValue = LitStr => ActionFn(8);
                let __sym0 = __pop_TermLitStr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExprValue(__nt), __end));
                9
            }
            38 => {
                // ExpressionNode = Id => ActionFn(14);
                let __sym0 = __pop_TermId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpressionNode(__nt), __end));
                10
            }
            39 => {
                // ExpressionNode = LitStr => ActionFn(15);
                let __sym0 = __pop_TermLitStr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpressionNode(__nt), __end));
                10
            }
            40 => {
                // NodeType = ComponentDefinitionNode => ActionFn(2);
                let __sym0 = __pop_NtComponentDefinitionNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNodeType(__nt), __end));
                11
            }
            41 => {
                // NodeType = ElementNode => ActionFn(3);
                let __sym0 = __pop_NtElementNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNodeType(__nt), __end));
                11
            }
            42 => {
                // NodeType = ExpressionNode => ActionFn(4);
                let __sym0 = __pop_NtExpressionNode(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNodeType(__nt), __end));
                11
            }
            43 => {
                // NodeType* =  => ActionFn(26);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action26::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtNodeType_2a(__nt), __end));
                12
            }
            44 => {
                // NodeType* = NodeType+ => ActionFn(27);
                let __sym0 = __pop_NtNodeType_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNodeType_2a(__nt), __end));
                12
            }
            45 => {
                // NodeType+ = NodeType => ActionFn(28);
                let __sym0 = __pop_NtNodeType(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNodeType_2b(__nt), __end));
                13
            }
            46 => {
                // NodeType+ = NodeType+, NodeType => ActionFn(29);
                let __sym1 = __pop_NtNodeType(__symbols);
                let __sym0 = __pop_NtNodeType_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action29::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtNodeType_2b(__nt), __end));
                13
            }
            47 => {
                // Param = Id, "=", ExprValue => ActionFn(9);
                let __sym2 = __pop_NtExprValue(__symbols);
                let __sym1 = __pop_Term_22_3d_22(__symbols);
                let __sym0 = __pop_TermId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action9::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtParam(__nt), __end));
                14
            }
            48 => {
                // Param? = Id, "=", ExprValue => ActionFn(37);
                let __sym2 = __pop_NtExprValue(__symbols);
                let __sym1 = __pop_Term_22_3d_22(__symbols);
                let __sym0 = __pop_TermId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action37::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtParam_3f(__nt), __end));
                15
            }
            49 => {
                // Param? =  => ActionFn(17);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action17::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtParam_3f(__nt), __end));
                15
            }
            50 => {
                // ParamList = Id, "=", ExprValue => ActionFn(60);
                let __sym2 = __pop_NtExprValue(__symbols);
                let __sym1 = __pop_Term_22_3d_22(__symbols);
                let __sym0 = __pop_TermId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action60::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtParamList(__nt), __end));
                16
            }
            51 => {
                // ParamList =  => ActionFn(61);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action61::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtParamList(__nt), __end));
                16
            }
            52 => {
                // ParamList = (<Param> ",")+, Id, "=", ExprValue => ActionFn(62);
                let __sym3 = __pop_NtExprValue(__symbols);
                let __sym2 = __pop_Term_22_3d_22(__symbols);
                let __sym1 = __pop_TermId(__symbols);
                let __sym0 = __pop_Nt_28_3cParam_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action62::<>(__sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtParamList(__nt), __end));
                16
            }
            53 => {
                // ParamList = (<Param> ",")+ => ActionFn(63);
                let __sym0 = __pop_Nt_28_3cParam_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action63::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtParamList(__nt), __end));
                16
            }
            54 => {
                // Template =  => ActionFn(58);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action58::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtTemplate(__nt), __end));
                17
            }
            55 => {
                // Template = NodeType+ => ActionFn(59);
                let __sym0 = __pop_NtNodeType_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action59::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTemplate(__nt), __end));
                17
            }
            56 => {
                // __Template = Template => ActionFn(0);
                let __sym0 = __pop_NtTemplate(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 19 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<'input>,())>
    ) -> ((), Token<'input>, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<'input>,())>
    ) -> ((), Token<'input>, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<'input>,())>
    ) -> ((), Token<'input>, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<'input>,())>
    ) -> ((), Token<'input>, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<'input>,())>
    ) -> ((), Token<'input>, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<'input>,())>
    ) -> ((), Token<'input>, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermComponentKeyword<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<'input>,())>
    ) -> ((), Token<'input>, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermComponentKeyword(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermId<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<'input>,())>
    ) -> ((), &'input str, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermId(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermLitStr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<'input>,())>
    ) -> ((), String, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermLitStr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_22_2c_22_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<'input>,())>
    ) -> ((), ::std::option::Option<Token<'input>>, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_22_2c_22_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cParam_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<'input>,())>
    ) -> ((), (String, ExprValue), ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cParam_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cParam_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<'input>,())>
    ) -> ((), ::std::vec::Vec<(String, ExprValue)>, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cParam_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cParam_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<'input>,())>
    ) -> ((), ::std::vec::Vec<(String, ExprValue)>, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cParam_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28Id_20_22_2c_22_3f_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<'input>,())>
    ) -> ((), (&'input str, ::std::option::Option<Token<'input>>), ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28Id_20_22_2c_22_3f_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28Id_20_22_2c_22_3f_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<'input>,())>
    ) -> ((), ::std::vec::Vec<(&'input str, ::std::option::Option<Token<'input>>)>, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28Id_20_22_2c_22_3f_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28Id_20_22_2c_22_3f_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<'input>,())>
    ) -> ((), ::std::vec::Vec<(&'input str, ::std::option::Option<Token<'input>>)>, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28Id_20_22_2c_22_3f_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComponentDefinitionNode<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<'input>,())>
    ) -> ((), NodeType, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComponentDefinitionNode(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtElementNode<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<'input>,())>
    ) -> ((), NodeType, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtElementNode(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExprValue<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<'input>,())>
    ) -> ((), ExprValue, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExprValue(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpressionNode<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<'input>,())>
    ) -> ((), NodeType, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpressionNode(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNodeType<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<'input>,())>
    ) -> ((), NodeType, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNodeType(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNodeType_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<'input>,())>
    ) -> ((), ::std::vec::Vec<NodeType>, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNodeType_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNodeType_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<'input>,())>
    ) -> ((), ::std::vec::Vec<NodeType>, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNodeType_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtParam<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<'input>,())>
    ) -> ((), (String, ExprValue), ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtParam(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtParam_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<'input>,())>
    ) -> ((), ::std::option::Option<(String, ExprValue)>, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtParam_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtParamList<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<'input>,())>
    ) -> ((), Vec<(String, ExprValue)>, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtParamList(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTemplate<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<'input>,())>
    ) -> ((), Vec<NodeType>, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTemplate(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Template<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<((),__Symbol<'input>,())>
    ) -> ((), Vec<NodeType>, ()) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Template(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Template::parse_Template;

fn __action0<
    'input,
>(
    (_, __0, _): ((), Vec<NodeType>, ()),
) -> Vec<NodeType>
{
    (__0)
}

fn __action1<
    'input,
>(
    (_, __0, _): ((), ::std::vec::Vec<NodeType>, ()),
) -> Vec<NodeType>
{
    (__0)
}

fn __action2<
    'input,
>(
    (_, __0, _): ((), NodeType, ()),
) -> NodeType
{
    (__0)
}

fn __action3<
    'input,
>(
    (_, __0, _): ((), NodeType, ()),
) -> NodeType
{
    (__0)
}

fn __action4<
    'input,
>(
    (_, __0, _): ((), NodeType, ()),
) -> NodeType
{
    (__0)
}

fn __action5<
    'input,
>(
    (_, _, _): ((), Token<'input>, ()),
    (_, name, _): ((), &'input str, ()),
    (_, _, _): ((), Token<'input>, ()),
    (_, children, _): ((), ::std::vec::Vec<NodeType>, ()),
    (_, _, _): ((), Token<'input>, ()),
) -> NodeType
{
    {
        let definition_name = name.into();

        NodeType::ComponentDefinitionNode(
            ComponentDefinitionType {
                name: definition_name,
                inputs: None,
                children: Some(children)
            }
        )
    }
}

fn __action6<
    'input,
>(
    (_, _, _): ((), Token<'input>, ()),
    (_, name, _): ((), &'input str, ()),
    (_, _, _): ((), Token<'input>, ()),
    (_, v, _): ((), ::std::vec::Vec<(&'input str, ::std::option::Option<Token<'input>>)>, ()),
    (_, _, _): ((), Token<'input>, ()),
    (_, _, _): ((), Token<'input>, ()),
    (_, children, _): ((), ::std::vec::Vec<NodeType>, ()),
    (_, _, _): ((), Token<'input>, ()),
) -> NodeType
{
    {
        let definition_name = name.into();
        let inputs = v.iter().map(|id| id.0.into()).collect();

        NodeType::ComponentDefinitionNode(
            ComponentDefinitionType {
                name: definition_name,
                inputs: Some(inputs),
                children: Some(children)
            }
        )
    }
}

fn __action7<
    'input,
>(
    (_, ident, _): ((), &'input str, ()),
) -> ExprValue
{
    ExprValue::VariableReference(ident.into())
}

fn __action8<
    'input,
>(
    (_, s, _): ((), String, ()),
) -> ExprValue
{
    ExprValue::LiteralString(s)
}

fn __action9<
    'input,
>(
    (_, ident, _): ((), &'input str, ()),
    (_, _, _): ((), Token<'input>, ()),
    (_, val, _): ((), ExprValue, ()),
) -> (String, ExprValue)
{
    {
        let ident = ident.into();
        
        (ident, val)
    }
}

fn __action10<
    'input,
>(
    (_, v, _): ((), ::std::vec::Vec<(String, ExprValue)>, ()),
    (_, e, _): ((), ::std::option::Option<(String, ExprValue)>, ()),
) -> Vec<(String, ExprValue)>
{
    {
        let mut v = v;
        if let Some(e) = e { v.push(e); }
        v
    }
}

fn __action11<
    'input,
>(
    (_, element_ty, _): ((), &'input str, ()),
    (_, _, _): ((), Token<'input>, ()),
    (_, children, _): ((), ::std::vec::Vec<NodeType>, ()),
    (_, _, _): ((), Token<'input>, ()),
) -> NodeType
{
    {
            let element_ty = element_ty.into();

            NodeType::ElementNode(
                ElementType {
                    element_ty: element_ty,
                    attrs: None,
                    children: Some(children)
                }
            )
        }
}

fn __action12<
    'input,
>(
    (_, element_ty, _): ((), &'input str, ()),
    (_, _, _): ((), Token<'input>, ()),
    (_, attrs, _): ((), Vec<(String, ExprValue)>, ()),
    (_, _, _): ((), Token<'input>, ()),
    (_, _, _): ((), Token<'input>, ()),
    (_, children, _): ((), ::std::vec::Vec<NodeType>, ()),
    (_, _, _): ((), Token<'input>, ()),
) -> NodeType
{
    {
            let element_ty = element_ty.into();

            NodeType::ElementNode(
                ElementType {
                    element_ty: element_ty,
                    attrs: Some(attrs),
                    children: Some(children)
                }
            )
        }
}

fn __action13<
    'input,
>(
    (_, element_ty, _): ((), &'input str, ()),
    (_, _, _): ((), Token<'input>, ()),
    (_, attrs, _): ((), Vec<(String, ExprValue)>, ()),
    (_, _, _): ((), Token<'input>, ()),
) -> NodeType
{
    {
            let element_ty = element_ty.into();

            NodeType::ElementNode(
                ElementType {
                    element_ty: element_ty,
                    attrs: Some(attrs),
                    children: None
                }
            )
        }
}

fn __action14<
    'input,
>(
    (_, ident, _): ((), &'input str, ()),
) -> NodeType
{
    {
            NodeType::ExpressionNode(ExprType::VariableReference(ident.into()))
        }
}

fn __action15<
    'input,
>(
    (_, value, _): ((), String, ()),
) -> NodeType
{
    {
            NodeType::ExpressionNode(ExprType::LiteralString(value.into()))
        }
}

fn __action16<
    'input,
>(
    (_, __0, _): ((), (String, ExprValue), ()),
) -> ::std::option::Option<(String, ExprValue)>
{
    Some(__0)
}

fn __action17<
    'input,
>(
    __lookbehind: &(),
    __lookahead: &(),
) -> ::std::option::Option<(String, ExprValue)>
{
    None
}

fn __action18<
    'input,
>(
    __lookbehind: &(),
    __lookahead: &(),
) -> ::std::vec::Vec<(String, ExprValue)>
{
    vec![]
}

fn __action19<
    'input,
>(
    (_, v, _): ((), ::std::vec::Vec<(String, ExprValue)>, ()),
) -> ::std::vec::Vec<(String, ExprValue)>
{
    v
}

fn __action20<
    'input,
>(
    (_, __0, _): ((), (String, ExprValue), ()),
    (_, _, _): ((), Token<'input>, ()),
) -> (String, ExprValue)
{
    (__0)
}

fn __action21<
    'input,
>(
    __lookbehind: &(),
    __lookahead: &(),
) -> ::std::vec::Vec<(&'input str, ::std::option::Option<Token<'input>>)>
{
    vec![]
}

fn __action22<
    'input,
>(
    (_, v, _): ((), ::std::vec::Vec<(&'input str, ::std::option::Option<Token<'input>>)>, ()),
) -> ::std::vec::Vec<(&'input str, ::std::option::Option<Token<'input>>)>
{
    v
}

fn __action23<
    'input,
>(
    (_, __0, _): ((), &'input str, ()),
    (_, __1, _): ((), ::std::option::Option<Token<'input>>, ()),
) -> (&'input str, ::std::option::Option<Token<'input>>)
{
    (__0, __1)
}

fn __action24<
    'input,
>(
    (_, __0, _): ((), Token<'input>, ()),
) -> ::std::option::Option<Token<'input>>
{
    Some(__0)
}

fn __action25<
    'input,
>(
    __lookbehind: &(),
    __lookahead: &(),
) -> ::std::option::Option<Token<'input>>
{
    None
}

fn __action26<
    'input,
>(
    __lookbehind: &(),
    __lookahead: &(),
) -> ::std::vec::Vec<NodeType>
{
    vec![]
}

fn __action27<
    'input,
>(
    (_, v, _): ((), ::std::vec::Vec<NodeType>, ()),
) -> ::std::vec::Vec<NodeType>
{
    v
}

fn __action28<
    'input,
>(
    (_, __0, _): ((), NodeType, ()),
) -> ::std::vec::Vec<NodeType>
{
    vec![__0]
}

fn __action29<
    'input,
>(
    (_, v, _): ((), ::std::vec::Vec<NodeType>, ()),
    (_, e, _): ((), NodeType, ()),
) -> ::std::vec::Vec<NodeType>
{
    { let mut v = v; v.push(e); v }
}

fn __action30<
    'input,
>(
    (_, __0, _): ((), (&'input str, ::std::option::Option<Token<'input>>), ()),
) -> ::std::vec::Vec<(&'input str, ::std::option::Option<Token<'input>>)>
{
    vec![__0]
}

fn __action31<
    'input,
>(
    (_, v, _): ((), ::std::vec::Vec<(&'input str, ::std::option::Option<Token<'input>>)>, ()),
    (_, e, _): ((), (&'input str, ::std::option::Option<Token<'input>>), ()),
) -> ::std::vec::Vec<(&'input str, ::std::option::Option<Token<'input>>)>
{
    { let mut v = v; v.push(e); v }
}

fn __action32<
    'input,
>(
    (_, __0, _): ((), (String, ExprValue), ()),
) -> ::std::vec::Vec<(String, ExprValue)>
{
    vec![__0]
}

fn __action33<
    'input,
>(
    (_, v, _): ((), ::std::vec::Vec<(String, ExprValue)>, ()),
    (_, e, _): ((), (String, ExprValue), ()),
) -> ::std::vec::Vec<(String, ExprValue)>
{
    { let mut v = v; v.push(e); v }
}

fn __action34<
    'input,
>(
    __0: ((), &'input str, ()),
    __1: ((), Token<'input>, ()),
) -> (&'input str, ::std::option::Option<Token<'input>>)
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action24(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action23(
        __0,
        __temp0,
    )
}

fn __action35<
    'input,
>(
    __0: ((), &'input str, ()),
) -> (&'input str, ::std::option::Option<Token<'input>>)
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action25(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action23(
        __0,
        __temp0,
    )
}

fn __action36<
    'input,
>(
    __0: ((), &'input str, ()),
    __1: ((), Token<'input>, ()),
    __2: ((), ExprValue, ()),
    __3: ((), Token<'input>, ()),
) -> (String, ExprValue)
{
    let __start0 = __0.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action9(
        __0,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action20(
        __temp0,
        __3,
    )
}

fn __action37<
    'input,
>(
    __0: ((), &'input str, ()),
    __1: ((), Token<'input>, ()),
    __2: ((), ExprValue, ()),
) -> ::std::option::Option<(String, ExprValue)>
{
    let __start0 = __0.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action9(
        __0,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action16(
        __temp0,
    )
}

fn __action38<
    'input,
>(
    __0: ((), &'input str, ()),
    __1: ((), Token<'input>, ()),
    __2: ((), ExprValue, ()),
    __3: ((), Token<'input>, ()),
) -> ::std::vec::Vec<(String, ExprValue)>
{
    let __start0 = __0.0.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action36(
        __0,
        __1,
        __2,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action32(
        __temp0,
    )
}

fn __action39<
    'input,
>(
    __0: ((), ::std::vec::Vec<(String, ExprValue)>, ()),
    __1: ((), &'input str, ()),
    __2: ((), Token<'input>, ()),
    __3: ((), ExprValue, ()),
    __4: ((), Token<'input>, ()),
) -> ::std::vec::Vec<(String, ExprValue)>
{
    let __start0 = __1.0.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action36(
        __1,
        __2,
        __3,
        __4,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action33(
        __0,
        __temp0,
    )
}

fn __action40<
    'input,
>(
    __0: ((), ::std::option::Option<(String, ExprValue)>, ()),
) -> Vec<(String, ExprValue)>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action18(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action10(
        __temp0,
        __0,
    )
}

fn __action41<
    'input,
>(
    __0: ((), ::std::vec::Vec<(String, ExprValue)>, ()),
    __1: ((), ::std::option::Option<(String, ExprValue)>, ()),
) -> Vec<(String, ExprValue)>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action19(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action10(
        __temp0,
        __1,
    )
}

fn __action42<
    'input,
>(
    __0: ((), &'input str, ()),
    __1: ((), Token<'input>, ()),
) -> ::std::vec::Vec<(&'input str, ::std::option::Option<Token<'input>>)>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action34(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action30(
        __temp0,
    )
}

fn __action43<
    'input,
>(
    __0: ((), &'input str, ()),
) -> ::std::vec::Vec<(&'input str, ::std::option::Option<Token<'input>>)>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action35(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action30(
        __temp0,
    )
}

fn __action44<
    'input,
>(
    __0: ((), ::std::vec::Vec<(&'input str, ::std::option::Option<Token<'input>>)>, ()),
    __1: ((), &'input str, ()),
    __2: ((), Token<'input>, ()),
) -> ::std::vec::Vec<(&'input str, ::std::option::Option<Token<'input>>)>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action34(
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action31(
        __0,
        __temp0,
    )
}

fn __action45<
    'input,
>(
    __0: ((), ::std::vec::Vec<(&'input str, ::std::option::Option<Token<'input>>)>, ()),
    __1: ((), &'input str, ()),
) -> ::std::vec::Vec<(&'input str, ::std::option::Option<Token<'input>>)>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action35(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action31(
        __0,
        __temp0,
    )
}

fn __action46<
    'input,
>(
    __0: ((), Token<'input>, ()),
    __1: ((), &'input str, ()),
    __2: ((), Token<'input>, ()),
    __3: ((), Token<'input>, ()),
    __4: ((), Token<'input>, ()),
    __5: ((), ::std::vec::Vec<NodeType>, ()),
    __6: ((), Token<'input>, ()),
) -> NodeType
{
    let __start0 = __2.2.clone();
    let __end0 = __3.0.clone();
    let __temp0 = __action21(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        __0,
        __1,
        __2,
        __temp0,
        __3,
        __4,
        __5,
        __6,
    )
}

fn __action47<
    'input,
>(
    __0: ((), Token<'input>, ()),
    __1: ((), &'input str, ()),
    __2: ((), Token<'input>, ()),
    __3: ((), ::std::vec::Vec<(&'input str, ::std::option::Option<Token<'input>>)>, ()),
    __4: ((), Token<'input>, ()),
    __5: ((), Token<'input>, ()),
    __6: ((), ::std::vec::Vec<NodeType>, ()),
    __7: ((), Token<'input>, ()),
) -> NodeType
{
    let __start0 = __3.0.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action22(
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        __0,
        __1,
        __2,
        __temp0,
        __4,
        __5,
        __6,
        __7,
    )
}

fn __action48<
    'input,
>(
    __0: ((), Token<'input>, ()),
    __1: ((), &'input str, ()),
    __2: ((), Token<'input>, ()),
    __3: ((), Token<'input>, ()),
) -> NodeType
{
    let __start0 = __2.2.clone();
    let __end0 = __3.0.clone();
    let __temp0 = __action26(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action5(
        __0,
        __1,
        __2,
        __temp0,
        __3,
    )
}

fn __action49<
    'input,
>(
    __0: ((), Token<'input>, ()),
    __1: ((), &'input str, ()),
    __2: ((), Token<'input>, ()),
    __3: ((), ::std::vec::Vec<NodeType>, ()),
    __4: ((), Token<'input>, ()),
) -> NodeType
{
    let __start0 = __3.0.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action27(
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action5(
        __0,
        __1,
        __2,
        __temp0,
        __4,
    )
}

fn __action50<
    'input,
>(
    __0: ((), Token<'input>, ()),
    __1: ((), &'input str, ()),
    __2: ((), Token<'input>, ()),
    __3: ((), Token<'input>, ()),
    __4: ((), Token<'input>, ()),
    __5: ((), Token<'input>, ()),
) -> NodeType
{
    let __start0 = __4.2.clone();
    let __end0 = __5.0.clone();
    let __temp0 = __action26(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action46(
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
        __5,
    )
}

fn __action51<
    'input,
>(
    __0: ((), Token<'input>, ()),
    __1: ((), &'input str, ()),
    __2: ((), Token<'input>, ()),
    __3: ((), Token<'input>, ()),
    __4: ((), Token<'input>, ()),
    __5: ((), ::std::vec::Vec<NodeType>, ()),
    __6: ((), Token<'input>, ()),
) -> NodeType
{
    let __start0 = __5.0.clone();
    let __end0 = __5.2.clone();
    let __temp0 = __action27(
        __5,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action46(
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
        __6,
    )
}

fn __action52<
    'input,
>(
    __0: ((), Token<'input>, ()),
    __1: ((), &'input str, ()),
    __2: ((), Token<'input>, ()),
    __3: ((), ::std::vec::Vec<(&'input str, ::std::option::Option<Token<'input>>)>, ()),
    __4: ((), Token<'input>, ()),
    __5: ((), Token<'input>, ()),
    __6: ((), Token<'input>, ()),
) -> NodeType
{
    let __start0 = __5.2.clone();
    let __end0 = __6.0.clone();
    let __temp0 = __action26(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action47(
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __temp0,
        __6,
    )
}

fn __action53<
    'input,
>(
    __0: ((), Token<'input>, ()),
    __1: ((), &'input str, ()),
    __2: ((), Token<'input>, ()),
    __3: ((), ::std::vec::Vec<(&'input str, ::std::option::Option<Token<'input>>)>, ()),
    __4: ((), Token<'input>, ()),
    __5: ((), Token<'input>, ()),
    __6: ((), ::std::vec::Vec<NodeType>, ()),
    __7: ((), Token<'input>, ()),
) -> NodeType
{
    let __start0 = __6.0.clone();
    let __end0 = __6.2.clone();
    let __temp0 = __action27(
        __6,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action47(
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __temp0,
        __7,
    )
}

fn __action54<
    'input,
>(
    __0: ((), &'input str, ()),
    __1: ((), Token<'input>, ()),
    __2: ((), Token<'input>, ()),
) -> NodeType
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action26(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action11(
        __0,
        __1,
        __temp0,
        __2,
    )
}

fn __action55<
    'input,
>(
    __0: ((), &'input str, ()),
    __1: ((), Token<'input>, ()),
    __2: ((), ::std::vec::Vec<NodeType>, ()),
    __3: ((), Token<'input>, ()),
) -> NodeType
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action27(
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action11(
        __0,
        __1,
        __temp0,
        __3,
    )
}

fn __action56<
    'input,
>(
    __0: ((), &'input str, ()),
    __1: ((), Token<'input>, ()),
    __2: ((), Vec<(String, ExprValue)>, ()),
    __3: ((), Token<'input>, ()),
    __4: ((), Token<'input>, ()),
    __5: ((), Token<'input>, ()),
) -> NodeType
{
    let __start0 = __4.2.clone();
    let __end0 = __5.0.clone();
    let __temp0 = __action26(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action12(
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
        __5,
    )
}

fn __action57<
    'input,
>(
    __0: ((), &'input str, ()),
    __1: ((), Token<'input>, ()),
    __2: ((), Vec<(String, ExprValue)>, ()),
    __3: ((), Token<'input>, ()),
    __4: ((), Token<'input>, ()),
    __5: ((), ::std::vec::Vec<NodeType>, ()),
    __6: ((), Token<'input>, ()),
) -> NodeType
{
    let __start0 = __5.0.clone();
    let __end0 = __5.2.clone();
    let __temp0 = __action27(
        __5,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action12(
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
        __6,
    )
}

fn __action58<
    'input,
>(
    __lookbehind: &(),
    __lookahead: &(),
) -> Vec<NodeType>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action26(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        __temp0,
    )
}

fn __action59<
    'input,
>(
    __0: ((), ::std::vec::Vec<NodeType>, ()),
) -> Vec<NodeType>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action27(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        __temp0,
    )
}

fn __action60<
    'input,
>(
    __0: ((), &'input str, ()),
    __1: ((), Token<'input>, ()),
    __2: ((), ExprValue, ()),
) -> Vec<(String, ExprValue)>
{
    let __start0 = __0.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action37(
        __0,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action40(
        __temp0,
    )
}

fn __action61<
    'input,
>(
    __lookbehind: &(),
    __lookahead: &(),
) -> Vec<(String, ExprValue)>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action17(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action40(
        __temp0,
    )
}

fn __action62<
    'input,
>(
    __0: ((), ::std::vec::Vec<(String, ExprValue)>, ()),
    __1: ((), &'input str, ()),
    __2: ((), Token<'input>, ()),
    __3: ((), ExprValue, ()),
) -> Vec<(String, ExprValue)>
{
    let __start0 = __1.0.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action37(
        __1,
        __2,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action41(
        __0,
        __temp0,
    )
}

fn __action63<
    'input,
>(
    __0: ((), ::std::vec::Vec<(String, ExprValue)>, ()),
) -> Vec<(String, ExprValue)>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action17(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action41(
        __0,
        __temp0,
    )
}

fn __action64<
    'input,
>(
    __0: ((), &'input str, ()),
    __1: ((), Token<'input>, ()),
    __2: ((), &'input str, ()),
    __3: ((), Token<'input>, ()),
    __4: ((), ExprValue, ()),
    __5: ((), Token<'input>, ()),
    __6: ((), Token<'input>, ()),
    __7: ((), Token<'input>, ()),
) -> NodeType
{
    let __start0 = __2.0.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action60(
        __2,
        __3,
        __4,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action56(
        __0,
        __1,
        __temp0,
        __5,
        __6,
        __7,
    )
}

fn __action65<
    'input,
>(
    __0: ((), &'input str, ()),
    __1: ((), Token<'input>, ()),
    __2: ((), Token<'input>, ()),
    __3: ((), Token<'input>, ()),
    __4: ((), Token<'input>, ()),
) -> NodeType
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action61(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action56(
        __0,
        __1,
        __temp0,
        __2,
        __3,
        __4,
    )
}

fn __action66<
    'input,
>(
    __0: ((), &'input str, ()),
    __1: ((), Token<'input>, ()),
    __2: ((), ::std::vec::Vec<(String, ExprValue)>, ()),
    __3: ((), &'input str, ()),
    __4: ((), Token<'input>, ()),
    __5: ((), ExprValue, ()),
    __6: ((), Token<'input>, ()),
    __7: ((), Token<'input>, ()),
    __8: ((), Token<'input>, ()),
) -> NodeType
{
    let __start0 = __2.0.clone();
    let __end0 = __5.2.clone();
    let __temp0 = __action62(
        __2,
        __3,
        __4,
        __5,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action56(
        __0,
        __1,
        __temp0,
        __6,
        __7,
        __8,
    )
}

fn __action67<
    'input,
>(
    __0: ((), &'input str, ()),
    __1: ((), Token<'input>, ()),
    __2: ((), ::std::vec::Vec<(String, ExprValue)>, ()),
    __3: ((), Token<'input>, ()),
    __4: ((), Token<'input>, ()),
    __5: ((), Token<'input>, ()),
) -> NodeType
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action63(
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action56(
        __0,
        __1,
        __temp0,
        __3,
        __4,
        __5,
    )
}

fn __action68<
    'input,
>(
    __0: ((), &'input str, ()),
    __1: ((), Token<'input>, ()),
    __2: ((), &'input str, ()),
    __3: ((), Token<'input>, ()),
    __4: ((), ExprValue, ()),
    __5: ((), Token<'input>, ()),
    __6: ((), Token<'input>, ()),
    __7: ((), ::std::vec::Vec<NodeType>, ()),
    __8: ((), Token<'input>, ()),
) -> NodeType
{
    let __start0 = __2.0.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action60(
        __2,
        __3,
        __4,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action57(
        __0,
        __1,
        __temp0,
        __5,
        __6,
        __7,
        __8,
    )
}

fn __action69<
    'input,
>(
    __0: ((), &'input str, ()),
    __1: ((), Token<'input>, ()),
    __2: ((), Token<'input>, ()),
    __3: ((), Token<'input>, ()),
    __4: ((), ::std::vec::Vec<NodeType>, ()),
    __5: ((), Token<'input>, ()),
) -> NodeType
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action61(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action57(
        __0,
        __1,
        __temp0,
        __2,
        __3,
        __4,
        __5,
    )
}

fn __action70<
    'input,
>(
    __0: ((), &'input str, ()),
    __1: ((), Token<'input>, ()),
    __2: ((), ::std::vec::Vec<(String, ExprValue)>, ()),
    __3: ((), &'input str, ()),
    __4: ((), Token<'input>, ()),
    __5: ((), ExprValue, ()),
    __6: ((), Token<'input>, ()),
    __7: ((), Token<'input>, ()),
    __8: ((), ::std::vec::Vec<NodeType>, ()),
    __9: ((), Token<'input>, ()),
) -> NodeType
{
    let __start0 = __2.0.clone();
    let __end0 = __5.2.clone();
    let __temp0 = __action62(
        __2,
        __3,
        __4,
        __5,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action57(
        __0,
        __1,
        __temp0,
        __6,
        __7,
        __8,
        __9,
    )
}

fn __action71<
    'input,
>(
    __0: ((), &'input str, ()),
    __1: ((), Token<'input>, ()),
    __2: ((), ::std::vec::Vec<(String, ExprValue)>, ()),
    __3: ((), Token<'input>, ()),
    __4: ((), Token<'input>, ()),
    __5: ((), ::std::vec::Vec<NodeType>, ()),
    __6: ((), Token<'input>, ()),
) -> NodeType
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action63(
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action57(
        __0,
        __1,
        __temp0,
        __3,
        __4,
        __5,
        __6,
    )
}

fn __action72<
    'input,
>(
    __0: ((), &'input str, ()),
    __1: ((), Token<'input>, ()),
    __2: ((), &'input str, ()),
    __3: ((), Token<'input>, ()),
    __4: ((), ExprValue, ()),
    __5: ((), Token<'input>, ()),
) -> NodeType
{
    let __start0 = __2.0.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action60(
        __2,
        __3,
        __4,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action13(
        __0,
        __1,
        __temp0,
        __5,
    )
}

fn __action73<
    'input,
>(
    __0: ((), &'input str, ()),
    __1: ((), Token<'input>, ()),
    __2: ((), Token<'input>, ()),
) -> NodeType
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action61(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action13(
        __0,
        __1,
        __temp0,
        __2,
    )
}

fn __action74<
    'input,
>(
    __0: ((), &'input str, ()),
    __1: ((), Token<'input>, ()),
    __2: ((), ::std::vec::Vec<(String, ExprValue)>, ()),
    __3: ((), &'input str, ()),
    __4: ((), Token<'input>, ()),
    __5: ((), ExprValue, ()),
    __6: ((), Token<'input>, ()),
) -> NodeType
{
    let __start0 = __2.0.clone();
    let __end0 = __5.2.clone();
    let __temp0 = __action62(
        __2,
        __3,
        __4,
        __5,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action13(
        __0,
        __1,
        __temp0,
        __6,
    )
}

fn __action75<
    'input,
>(
    __0: ((), &'input str, ()),
    __1: ((), Token<'input>, ()),
    __2: ((), ::std::vec::Vec<(String, ExprValue)>, ()),
    __3: ((), Token<'input>, ()),
) -> NodeType
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action63(
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action13(
        __0,
        __1,
        __temp0,
        __3,
    )
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<((),Token<'input>,()),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for Token<'input> {
    type Error = ();
    fn to_triple(value: Self) -> Result<((),Token<'input>,()),()> {
        Ok(((), value, ()))
    }
}
impl<'input, > __ToTriple<'input, > for Result<(Token<'input>),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<((),Token<'input>,()),()> {
        value.map(|v| ((), v, ()))
    }
}
