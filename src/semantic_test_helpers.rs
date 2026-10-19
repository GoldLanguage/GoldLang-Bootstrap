use crate::ast::{
    AST, Expr, Stmt, GlobalStmt, Type, Constant, IntLiteralValue, Span,
    Function, Param, VariableDeclaration, VariableAssignment,
    BreakStmt
};

use crate::tests_consts::{
    ALL_BIN_OP_KIND_COMP, ALL_BIN_OP_KIND_COMP_EQ, ALL_BIN_OP_KIND_BIT_ARTH
};

use std::sync::LazyLock;

// Helper functions for all the semantics analysis layer test submodules
//

// All types with dynamic array types 
pub static ALL_TYPES_WITH_DYN_ARR: LazyLock<Vec<Type>> = LazyLock::new(|| {
    vec![
        Type::Int8,
        Type::Int16,
        Type::Int32,
        Type::Int64,
        Type::Int128,
        Type::Byte,
        Type::Uint16,
        Type::Uint32,
        Type::Uint64,
        Type::Uint128,
        Type::Usize,
        Type::Float64,
        Type::Bool,
        Type::Char,
        Type::String,

        Type::Array(Box::new(Type::Int8)),
        Type::Array(Box::new(Type::Int16)),
        Type::Array(Box::new(Type::Int32)),
        Type::Array(Box::new(Type::Int64)),
        Type::Array(Box::new(Type::Int128)),

        Type::Array(Box::new(Type::Byte)),
        Type::Array(Box::new(Type::Uint16)),
        Type::Array(Box::new(Type::Uint32)),
        Type::Array(Box::new(Type::Uint64)),
        Type::Array(Box::new(Type::Uint128)),
        Type::Array(Box::new(Type::Usize)),

        Type::Array(Box::new(Type::Float64)),
        Type::Array(Box::new(Type::Bool)),
        Type::Array(Box::new(Type::Char)),
        Type::Array(Box::new(Type::String)),
    ]
});

// All types, with only few integers (signed, and unsigned) with dynamic array types 
//


pub static ALL_TYPES_FEW_INTS_WITH_DYN_ARR: LazyLock<Vec<Type>> = LazyLock::new(|| {
    vec![
        Type::Uint128,
        Type::Int128,
        Type::Float64,
        Type::Bool,
        Type::Char,
        Type::String,

        Type::Array(Box::new(Type::Uint128)),
        Type::Array(Box::new(Type::Int128)),
        Type::Array(Box::new(Type::Float64)),
        Type::Array(Box::new(Type::Bool)),
        Type::Array(Box::new(Type::Char)),
        Type::Array(Box::new(Type::String)),
    ]
});

pub static ALL_TYPES_FEW_INTS_WITH_DYN_ARR_SCATTERED: LazyLock<Vec<Type>> = LazyLock::new(|| {
    vec![
        Type::Array(Box::new(Type::Bool)),
        Type::Char,
        Type::Bool,
        Type::Uint128,
        Type::Array(Box::new(Type::Float64)),
        Type::Array(Box::new(Type::Char)),
        Type::Float64,
        Type::Array(Box::new(Type::Uint128)),
        Type::Array(Box::new(Type::String)),
        Type::Array(Box::new(Type::Int128)),
        Type::Int128,
        Type::String
    ]
});



pub fn get_many_boolean_conditions() -> Vec<Expr> {
    let literals = get_all_literals();
    
    let mut boolean_conds = vec![
        bool_lit(true),
        bool_lit(false),
    ];

    for l in literals {
        for b in ALL_BIN_OP_KIND_COMP {
            // So that >= > <= < doesnt get performed on non integer/floats.
            if !ALL_BIN_OP_KIND_COMP_EQ.contains(&b) {
                match l {
                    Expr::StringLiteral { .. } | Expr::CharLiteral { .. } | Expr::BoolLiteral { .. } | Expr::ArrayLiteral { .. } => {
                        continue
                    },
                    _ => {}
                }
            }

            let bin = Expr::BinOp {
                left: Box::new(l.clone()),
                right: Box::new(l.clone()),
                op: b,
                span: span(),
            };

            boolean_conds.push(bin);
        }
    }

    return boolean_conds;
}

pub fn get_many_boolean_conditions_no_dyn_arr() -> Vec<Expr> {
    let literals = get_all_literals_no_arr();
    
    let mut boolean_conds = vec![
        bool_lit(true),
        bool_lit(false),
    ];

    for l in literals {
        for b in ALL_BIN_OP_KIND_COMP {
            // So that >= > <= < doesnt get performed on non integer/floats.
            if !ALL_BIN_OP_KIND_COMP_EQ.contains(&b) {
                match l {
                    Expr::StringLiteral { .. } | Expr::BoolLiteral { .. } | Expr::ArrayLiteral { .. } => {
                        continue
                    },
                    _ => {}
                }
            }

            let bin = Expr::BinOp {
                left: Box::new(l.clone()),
                right: Box::new(l.clone()),
                op: b,
                span: span(),
            };

            boolean_conds.push(bin);
        }
    }

    return boolean_conds;
}


pub fn get_non_boolean_conditions() -> Vec<Expr> {
    let literals = get_all_literals();
    
    let mut non_boolean_conds = vec![];

    for l in literals {
        if matches!(l, Expr::BoolLiteral { .. }) {
            continue
        }
        non_boolean_conds.push(l.clone());

        for b in ALL_BIN_OP_KIND_BIT_ARTH {
            if !matches!(l, Expr::IntLiteral { .. }) {
                continue
            }
            let bin = Expr::BinOp {
                left: Box::new(l.clone()),
                right: Box::new(l.clone()),
                op: b,
                span: span(),
            };
            non_boolean_conds.push(bin);
        }
    }

    return non_boolean_conds;
}


pub fn get_all_literals_no_arr_bool() -> [Expr; 14] {
    return [
        int8_lit(1),
        int16_lit(1),
        int32_lit(1),
        int64_lit(1),
        int128_lit(1),

        byte_lit(1),
        uint16_lit(1),
        uint32_lit(1),
        uint64_lit(1),
        uint128_lit(1),

        usize_lit(1),

        float64_lit(1.0),

        char_lit('f'),
        str_lit("Hi")
    ]
}


pub fn get_all_literals_no_arr_no_ints() -> [Expr; 4] {
    let literals = [

        float64_lit(1.0),

        bool_lit(false),
        char_lit('f'),
        str_lit("Hi")
    ];

    return literals;
}

pub fn get_all_literals_few_ints() -> [Expr; 12] {
    [
        uint128_lit(1),
        int128_lit(1),

        float64_lit(1.0),

        bool_lit(false),
        char_lit('f'),
        str_lit("Hi"),

        array_lit(vec![uint128_lit(1), uint128_lit(u128::MIN), uint128_lit(u128::MAX) ], Some(Type::Array(Box::new(Type::Uint128)))),
        array_lit(vec![int128_lit(1), int128_lit(i128::MIN), int128_lit(i128::MAX) ], Some(Type::Array(Box::new(Type::Int128)))),
        array_lit(vec![float64_lit(1.0), float64_lit(f64::MIN), float64_lit(f64::MAX) ], Some(Type::Array(Box::new(Type::Float64)))),
        
        array_lit(vec![bool_lit(false), bool_lit(true) ], Some(Type::Array(Box::new(Type::Float64)))),
        array_lit(vec![char_lit('\n'), char_lit('H'), char_lit('!')], Some(Type::Array(Box::new(Type::Char)))),
        array_lit(vec![str_lit(""), str_lit("Hi"), str_lit(" !")], Some(Type::Array(Box::new(Type::String))))
    ]
}


pub fn get_all_literals_few_ints_scattered() -> [Expr; 12] {
    [
        array_lit(vec![bool_lit(false), bool_lit(true) ], Some(Type::Array(Box::new(Type::Bool)))),
        char_lit('f'),

        bool_lit(false),
        uint128_lit(1),
        array_lit(vec![float64_lit(1.0), float64_lit(f64::MIN), float64_lit(f64::MAX) ], Some(Type::Array(Box::new(Type::Float64)))),
        array_lit(vec![char_lit('\n'), char_lit('H'), char_lit('!')], Some(Type::Array(Box::new(Type::Char)))),
        float64_lit(1.0),

        array_lit(vec![uint128_lit(1), uint128_lit(u128::MIN), uint128_lit(u128::MAX) ], Some(Type::Array(Box::new(Type::Uint128)))),
        array_lit(vec![str_lit(""), str_lit("Hi"), str_lit(" !")], Some(Type::Array(Box::new(Type::String)))),
        array_lit(vec![int128_lit(1), int128_lit(i128::MIN), int128_lit(i128::MAX) ], Some(Type::Array(Box::new(Type::Int128)))),
        int128_lit(1),
        str_lit("Hi")
    ]
}

pub fn get_all_literals_no_arr_few_ints() -> [Expr; 6] {
    let literals = [
        uint128_lit(1),
        int128_lit(1),

        float64_lit(1.0),

        bool_lit(false),
        char_lit('f'),
        str_lit("Hi")
    ];

    return literals;
}


pub fn get_all_literals_no_arr_few_ints_scattered() -> [Expr; 6] {
    let literals = [
        str_lit("Hi"),

        bool_lit(false),
        int128_lit(1),
        char_lit('f'),
        float64_lit(1.0),
        uint128_lit(1),
    ];

    return literals;
}


pub fn get_all_signed_literals_no_arr() -> [Expr; 6] {
    let literals = [
        int8_lit(1),
        int16_lit(1),
        int32_lit(1),
        int64_lit(1),
        int128_lit(1),

        float64_lit(1.0),
    ];

    return literals;
}


pub fn get_all_signed_literals_no_arr_no_float() -> [Expr; 5] {
    let literals = [
        int8_lit(1),
        int16_lit(1),
        int32_lit(1),
        int64_lit(1),
        int128_lit(1),
    ];

    return literals;
}



pub fn get_all_unsigned_literals_no_arr() -> [Expr; 6] {
    let literals = [
        byte_lit(1),
        uint16_lit(1),
        uint32_lit(1),
        uint64_lit(1),
        uint128_lit(1),
        usize_lit(1)
    ];

    return literals;
}


pub fn get_all_literals_no_arr_str_bool() -> [Expr; 12] {
    let literals = [
        int8_lit(1),
        int16_lit(1),
        int32_lit(1),
        int64_lit(1),
        int128_lit(1),

        byte_lit(1),
        uint16_lit(1),
        uint32_lit(1),
        uint64_lit(1),
        uint128_lit(1),

        usize_lit(1),

        float64_lit(1.0),
    ];

    return literals;
}



pub fn get_all_literals_no_arr_str_bool_scattered() -> [Expr; 12] {
    let literals = [
        uint32_lit(1),
        int8_lit(1),
        int64_lit(1),
        uint128_lit(1),

        uint16_lit(1),
        usize_lit(1),
        int16_lit(1),
        byte_lit(1),
        float64_lit(1.0),
        uint64_lit(1),
        int128_lit(1),
        int32_lit(1),

    ];

    return literals;
}




pub fn get_all_literals_no_arr_str_bool_float() -> [Expr; 11] {
    let literals = [
        int8_lit(1),
        int16_lit(1),
        int32_lit(1),
        int64_lit(1),
        int128_lit(1),

        byte_lit(1),
        uint16_lit(1),
        uint32_lit(1),
        uint64_lit(1),
        uint128_lit(1),

        usize_lit(1),
    ];

    return literals;
}

pub fn get_all_literals_no_arr() -> [Expr; 15] {
    [
        int8_lit(1),
        int16_lit(1),
        int32_lit(1),
        int64_lit(1),
        int128_lit(1),

        byte_lit(1),
        uint16_lit(1),
        uint32_lit(1),
        uint64_lit(1),
        uint128_lit(1),

        usize_lit(1),

        float64_lit(1.0),

        bool_lit(false),
        char_lit('f'),
        str_lit("Hi")
    ]
}

pub fn get_all_literals_no_arr_scattered_order() -> [Expr; 15] {
    [
        int128_lit(1),
        int8_lit(1),
        uint64_lit(1),
        uint16_lit(1),
        int64_lit(1),
        str_lit("Hi"),
        uint128_lit(1),
        float64_lit(1.0),
        uint32_lit(1),
        char_lit('f'),
        int16_lit(1),
        bool_lit(false),
        byte_lit(1),
        int32_lit(1),
        usize_lit(1)
    ]
}



pub fn get_all_literals_no_arr_no_usize() -> [Expr; 14] {
    return [
        int8_lit(1),
        int16_lit(1),
        int32_lit(1),
        int64_lit(1),
        int128_lit(1),

        byte_lit(1),
        uint16_lit(1),
        uint32_lit(1),
        uint64_lit(1),
        uint128_lit(1),

        float64_lit(1.0),

        bool_lit(false),
        char_lit('f'),
        str_lit("Hi")
    ]
}

pub fn get_all_literals() -> [Expr; 30] {
    return [
        int8_lit(1),
        int16_lit(1),
        int32_lit(1),
        int64_lit(1),
        int128_lit(1),

        byte_lit(1),
        uint16_lit(1),
        uint32_lit(1),
        uint64_lit(1),
        uint128_lit(1),

        usize_lit(1),

        float64_lit(1.0),

        bool_lit(false),
        char_lit('f'),
        str_lit("Hi"),

        array_lit(vec![int8_lit(1), int8_lit(i8::MIN), int8_lit(i8::MAX) ], Some(Type::Array(Box::new(Type::Int8)))),
        array_lit(vec![int16_lit(1), int16_lit(i16::MIN), int16_lit(i16::MAX) ], Some(Type::Array(Box::new(Type::Int16)))),
        array_lit(vec![int32_lit(1), int32_lit(i32::MIN), int32_lit(i32::MAX) ], Some(Type::Array(Box::new(Type::Int32)))),
        array_lit(vec![int64_lit(1), int64_lit(i64::MIN), int64_lit(i64::MAX) ], Some(Type::Array(Box::new(Type::Int64)))),
        array_lit(vec![int128_lit(1), int128_lit(i128::MIN), int128_lit(i128::MAX) ], Some(Type::Array(Box::new(Type::Int128)))),

        array_lit(vec![byte_lit(1), byte_lit(u8::MIN), byte_lit(u8::MAX) ], Some(Type::Array(Box::new(Type::Byte)))),
        array_lit(vec![uint16_lit(1), uint16_lit(u16::MIN), uint16_lit(u16::MAX) ], Some(Type::Array(Box::new(Type::Uint16)))),
        array_lit(vec![uint32_lit(1), uint32_lit(u32::MIN), uint32_lit(u32::MAX) ], Some(Type::Array(Box::new(Type::Uint32)))),
        array_lit(vec![uint64_lit(1), uint64_lit(u64::MIN), uint64_lit(u64::MAX) ], Some(Type::Array(Box::new(Type::Uint64)))),
        array_lit(vec![uint128_lit(1), uint128_lit(u128::MIN), uint128_lit(u128::MAX) ], Some(Type::Array(Box::new(Type::Uint128)))),
        array_lit(vec![usize_lit(1), usize_lit(usize::MIN), usize_lit(usize::MAX) ], Some(Type::Array(Box::new(Type::Usize)))),

        array_lit(vec![float64_lit(1.0), float64_lit(f64::MIN), float64_lit(f64::MAX) ], Some(Type::Array(Box::new(Type::Float64)))),
        array_lit(vec![bool_lit(false), bool_lit(true)], Some(Type::Array(Box::new(Type::Bool)))),
        
        array_lit(vec![char_lit('\n'), char_lit('H'), char_lit('!')], Some(Type::Array(Box::new(Type::Char)))),
        array_lit(vec![str_lit(""), str_lit("Hi"), str_lit(" !")], Some(Type::Array(Box::new(Type::String))))
    ];
}

pub fn get_all_literals_with_var_and_var_arr() -> [Expr; 32] {
    return [
        int8_lit(1),
        int16_lit(1),
        int32_lit(1),
        int64_lit(1),
        int128_lit(1),

        byte_lit(1),
        uint16_lit(1),
        uint32_lit(1),
        uint64_lit(1),
        uint128_lit(1),

        usize_lit(1),

        float64_lit(1.0),

        bool_lit(false),
        char_lit('f'),
        str_lit("Hi"),

        var_expr("x"),

        array_lit(vec![int8_lit(1), int8_lit(i8::MIN), int8_lit(i8::MAX) ], Some(Type::Array(Box::new(Type::Int8)))),
        array_lit(vec![int16_lit(1), int16_lit(i16::MIN), int16_lit(i16::MAX) ], Some(Type::Array(Box::new(Type::Int16)))),
        array_lit(vec![int32_lit(1), int32_lit(i32::MIN), int32_lit(i32::MAX) ], Some(Type::Array(Box::new(Type::Int32)))),
        array_lit(vec![int64_lit(1), int64_lit(i64::MIN), int64_lit(i64::MAX) ], Some(Type::Array(Box::new(Type::Int64)))),
        array_lit(vec![int128_lit(1), int128_lit(i128::MIN), int128_lit(i128::MAX) ], Some(Type::Array(Box::new(Type::Int128)))),

        array_lit(vec![byte_lit(1), byte_lit(u8::MIN), byte_lit(u8::MAX) ], Some(Type::Array(Box::new(Type::Byte)))),
        array_lit(vec![uint16_lit(1), uint16_lit(u16::MIN), uint16_lit(u16::MAX) ], Some(Type::Array(Box::new(Type::Uint16)))),
        array_lit(vec![uint32_lit(1), uint32_lit(u32::MIN), uint32_lit(u32::MAX) ], Some(Type::Array(Box::new(Type::Uint32)))),
        array_lit(vec![uint64_lit(1), uint64_lit(u64::MIN), uint64_lit(u64::MAX) ], Some(Type::Array(Box::new(Type::Uint64)))),
        array_lit(vec![uint128_lit(1), uint128_lit(u128::MIN), uint128_lit(u128::MAX) ], Some(Type::Array(Box::new(Type::Uint128)))),
        array_lit(vec![usize_lit(1), usize_lit(usize::MIN), usize_lit(usize::MAX) ], Some(Type::Array(Box::new(Type::Usize)))),

        array_lit(vec![float64_lit(1.0), float64_lit(f64::MIN), float64_lit(f64::MAX) ], Some(Type::Array(Box::new(Type::Float64)))),
        array_lit(vec![bool_lit(false), bool_lit(true)], Some(Type::Array(Box::new(Type::Bool)))),
        
        array_lit(vec![char_lit('\n'), char_lit('H'), char_lit('!')], Some(Type::Array(Box::new(Type::Char)))),
        array_lit(vec![str_lit(""), str_lit("Hi"), str_lit(" !")], Some(Type::Array(Box::new(Type::String)))),
        
        array_lit(vec![var_expr("x"), var_expr("y"), var_expr("z")], Some(Type::Array(Box::new(Type::String)))),

    ];
}





pub fn span() -> Span {
    Span { line: 1, column: 0 }
}

/// Build an AST that contains exactly one function.
pub fn ast_one(func: Function) -> AST {
    AST { functions: vec![func], globals: vec![] }
}

/// Build a void function (no return type) with the given body.
pub fn void_func(name: &str, params: Vec<Param>, mut body: Vec<Stmt>) -> Function {
    if body.len() == 0 {
        // Dummy body because empty branches are not allowed.
        body = vec![var_decl(true, "x", Type::Int8, int32_lit(69))];
    }

    Function {
        name: name.to_string(),
        params,
        return_type: None,
        body,
        span: span(),
    }
}

pub fn returning_func(name: &str, params: Vec<Param>, ret: Vec<Type>, body: Vec<Stmt>) -> Function {
    Function {
        name: name.to_string(),
        params,
        return_type: Some(ret),
        body,
        span: span(),
    }
}

pub fn param(name: &str, ty: Type) -> Param {
    Param { name: name.to_string(), type_name: ty, span: span() }
}


pub fn const_define_locally(name: &str, ty: Type, value: Expr) -> Stmt {
    Stmt::Const(Constant {
        name: name.to_string(),
        type_name: ty,
        value,
        span: span(),
    })
}

pub fn const_define_globally(name: &str, ty: Type, value: Expr) -> GlobalStmt {
    GlobalStmt::Const(Constant {
        name: name.to_string(),
        type_name: ty,
        value,
        span: span(),
    })
}

pub fn var_decl(explicitly_initialized: bool, name: &str, ty: Type, value: Expr) -> Stmt {
    Stmt::VarDecl(VariableDeclaration {
        name: name.to_string(),
        type_name: ty,
        value,
        explicitly_initialized,
        span: span(),
    })
}


pub fn var_assign(name: &str, value: Expr) -> Stmt {
    Stmt::VarAssign(VariableAssignment {
        name: name.to_string(),
        value,
        span: span(),
    })
}

pub fn break_stmt() -> Stmt {
    Stmt::Break(BreakStmt {
        span: span(),
    })
}

pub fn contains_array_literal(expr: &Expr) -> bool {
    match expr {
        Expr::ArrayLiteral { .. } => true,
        Expr::BinOp { left, right, .. } => {
            contains_array_literal(left) || contains_array_literal(right)
        }
        _ => false,
    }
}


pub fn array_lit(exprs: Vec<Expr>, type_name: Option<Type>) -> Expr {
    Expr::ArrayLiteral { elements: exprs, type_name, span: span() }
}

pub fn int8_lit(n: i8) -> Expr {
    Expr::IntLiteral { value: IntLiteralValue::Int8(n), span: span() }
}

pub fn int16_lit(n: i16) -> Expr {
    Expr::IntLiteral { value: IntLiteralValue::Int16(n), span: span() }
}

pub fn int32_lit(n: i32) -> Expr {
    Expr::IntLiteral { value: IntLiteralValue::Int32(n), span: span() }
}

pub fn int64_lit(n: i64) -> Expr {
    Expr::IntLiteral { value: IntLiteralValue::Int64(n), span: span() }
}

pub fn int128_lit(n: i128) -> Expr {
    Expr::IntLiteral { value: IntLiteralValue::Int128(n), span: span() }
}



pub fn byte_lit(b: u8) -> Expr {
    Expr::IntLiteral { value: IntLiteralValue::Byte(b), span: span() }
}

pub fn uint16_lit(n: u16) -> Expr {
    Expr::IntLiteral { value: IntLiteralValue::Uint16(n), span: span() }
}

pub fn uint32_lit(n: u32) -> Expr {
    Expr::IntLiteral { value: IntLiteralValue::Uint32(n), span: span() }
}

pub fn uint64_lit(n: u64) -> Expr {
    Expr::IntLiteral { value: IntLiteralValue::Uint64(n), span: span() }
}

pub fn uint128_lit(n: u128) -> Expr {
    Expr::IntLiteral { value: IntLiteralValue::Uint128(n), span: span() }
}


pub fn usize_lit(n: usize) -> Expr {
    Expr::IntLiteral { value: IntLiteralValue::Usize(n), span: span() }
}



pub fn float64_lit(f: f64) -> Expr {
    Expr::Float64Literal { value: f, span: span() }
}


pub fn bool_lit(b: bool) -> Expr {
    Expr::BoolLiteral { value: b, span: span() }
}

pub fn char_lit(c: char) -> Expr {
    Expr::CharLiteral { value: c, span: span() }
}

pub fn str_lit(s: &str) -> Expr {
    Expr::StringLiteral { value: s.to_string(), span: span() }
}

pub fn var_expr(name: &str) -> Expr {
    Expr::Var { name: name.to_string(), span: span() }
}

pub fn call_expr(name: &str, args: Vec<Expr>) -> Expr {
    Expr::Call { name: name.to_string(), args, span: span() }
}

pub fn return_stmt(exprs: Vec<Expr>) -> Stmt {
    Stmt::Return(exprs)
}

