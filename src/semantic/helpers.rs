use super::{
    Type, 
    Stmt, 
    Span, 
    Expr, 
    GoldError,

    BindingInfo,
    HashMap
};

use crate::ast::{
    IntLiteralValue
};


/// Takes 2 integer types, determines which type can hold more than the other
///
pub fn get_bigger_type_of_two_integers(t_1: Type, t_2: Type) -> Type {
    assert!(
        !(!t_1.is_integer_type() || !t_2.is_integer_type()), 
        "(Compiler bug) you should not call this function unless you are sure both types are integer type. We got {t_1:?} {t_2:?}"
    );

    let t_1_score = match t_1 {
            Type::Int8 => 1,
            Type::Int16 => 3,
            Type::Int32 => 5,
            Type::Int64 => 7,
            Type::Int128 => 9,

            Type::Byte => 2,
            Type::Uint16 => 4,
            Type::Uint32 => 6,
            Type::Uint64 | Type::Usize => 8,
            Type::Uint128 => 10,
            
            other => panic!("Shouldve been an integer, instead its {other:?}")
    };

    let t_2_score = match t_2 {
            Type::Int8 => 1,
            Type::Int16 => 3,
            Type::Int32 => 5,
            Type::Int64 => 7,
            Type::Int128 => 9,

            Type::Byte => 2,
            Type::Uint16 => 4,
            Type::Uint32 => 6,
            Type::Uint64 | Type::Usize => 8,
            Type::Uint128 => 10,

            other => panic!("Shouldve been an integer, instead its {other:?}")
    };


    if t_1_score > t_2_score {
        t_1
    } else {
        t_2
    }
}


pub fn check_identifier_is_already_taken(
    identifier: &str,
    span: &Span,
    locals: &mut HashMap<String, BindingInfo>, 
    fun_sigs: &HashMap<String, (Vec<Type>, Option<Vec<Type>>)>
) -> Result<(), GoldError> {
    if fun_sigs.contains_key(identifier) {
        return Err(GoldError::Semantic(format!(
                    "Variable identifier name `{}` is already taken by a function. (line {} column {})", 
                    identifier, span.line, span.column
                )))
    }

    if locals.contains_key(identifier) {
        return Err(GoldError::Semantic(format!(
                "Variable `{}` is already declared, overshadowing is not allowed. (line {} column {})", 
                identifier, span.line, span.column
            )))
    }

    Ok(())
}


// helper to get the span of a statement (so we can point to offending code)
pub fn stmt_span(s: &Stmt) -> Span {
    match s {
        Stmt::Const(c) => c.span,
        Stmt::VarDecl(v) => v.span,
        Stmt::VarAssign(a) => a.span,
        Stmt::Expr(e) => expr_span(e),
        Stmt::Return(e) | Stmt::Lock(e) | Stmt::Unlock(e) => expr_span(&e[0]),

        Stmt::For(f) => f.span,
        Stmt::While(w) => w.span,
        Stmt::Break(b) => b.span,
        Stmt::Continue(c) => c.span,
        Stmt::Infinite(i) => i.span,
        Stmt::If(i) => i.span,
        Stmt::VarDeclMulti(_, v) => expr_span(v), 
        Stmt::VarAssignMulti(ma) => ma.span,
    }
}

// helper to get spanof a expr
pub const fn expr_span(e: &Expr) -> Span {
    match e {
        Expr::IntLiteral { span, .. } |
        Expr::Float64Literal { span, .. } |
        Expr::BoolLiteral { span, .. } |
        Expr::ArrayLiteral { span, .. } |
        Expr::CharLiteral { span, .. } |
        Expr::StringLiteral { span, .. } |

        Expr::ArrayAccess { span, .. } |
        Expr::ArraySlicing { span, .. } |
        Expr::Var { span, .. } |
        Expr::BinOp { span, .. } |
        Expr::UnaryOp { span, .. } |
        Expr::Call { span, .. } |
        Expr::CopyCall { span, .. } | 
        Expr::FormatCall { span, .. } |
        Expr::RangeCall { span, .. } => *span,
    }
}


pub fn coerce_integer_literal_to_type_helper(expected_ty: Type, value: IntLiteralValue, span: Span) -> Result<IntLiteralValue, GoldError> {
    assert!(value.get_type().is_integer_type(), "(Compiler bug) Value `{}` of type `{}` is not an integer type", value, value.get_type());

    let range_err = || GoldError::Semantic(format!(
        "Integer literal `{}` out of range for type `{}` (line {} column {})",
        value, expected_ty, span.line, span.column
    ));

    // Normalize up front. One or both may be None if the value can't be represented that way.
    let (as_signed, as_unsigned) = if value.is_signed() {
        let s = value.as_i128();
        let u = if s >= 0 { Some(s.cast_unsigned()) } else { None };
        (Some(s), u)

    } else {
        let u = value.as_u128();
        let s = if u <= i128::MAX as u128 { Some(u.cast_signed()) } else { None };
        (s, Some(u))
    };

    let fits_signed   = |min: i128, max: i128| as_signed.filter(|&v| v >= min && v <= max).ok_or_else(range_err);
    let fits_unsigned = |max: u128|            as_unsigned.filter(|&v| v <= max).ok_or_else(range_err);

    #[expect(clippy::cast_lossless, reason = "I think casting here should be ok as we did checks above")]
    #[expect(clippy::cast_possible_truncation, reason = "I think casting here should be ok as we did checks above")]
    match expected_ty {
        Type::Int8   => Ok(IntLiteralValue::Int8  (fits_signed(i8::MIN   as i128, i8::MAX   as i128)? as i8)),
        Type::Int16  => Ok(IntLiteralValue::Int16 (fits_signed(i16::MIN  as i128, i16::MAX  as i128)? as i16)),
        Type::Int32  => Ok(IntLiteralValue::Int32 (fits_signed(i32::MIN  as i128, i32::MAX  as i128)? as i32)),
        Type::Int64  => Ok(IntLiteralValue::Int64 (fits_signed(i64::MIN  as i128, i64::MAX  as i128)? as i64)),
        Type::Int128 => Ok(IntLiteralValue::Int128(fits_signed(i128::MIN,         i128::MAX)?)),

        Type::Byte   => Ok(IntLiteralValue::Byte  (fits_unsigned(u8::MAX   as u128)? as u8)),
        Type::Uint16 => Ok(IntLiteralValue::Uint16(fits_unsigned(u16::MAX  as u128)? as u16)),
        Type::Uint32 => Ok(IntLiteralValue::Uint32(fits_unsigned(u32::MAX  as u128)? as u32)),
        Type::Uint64 => Ok(IntLiteralValue::Uint64(fits_unsigned(u64::MAX  as u128)? as u64)),
        Type::Uint128 => Ok(IntLiteralValue::Uint128(fits_unsigned(u128::MAX)?)),
        Type::Usize  => Ok(IntLiteralValue::Usize (fits_unsigned(usize::MAX as u128)? as usize)),

        other => panic!("(Compiler bug) Unexpected type in infer_integer_literal_helper: {other:?}")
    }
}
