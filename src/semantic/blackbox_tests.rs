use super::*;
use crate::semantic_test_helpers::*;
use crate::ast::{
    Expr, Type, FixedArraySize, IntLiteralValue, ArraySliceRange,
    UnaryOpKind, BinOpKind,
    MultiVariableDeclaration, MultiAssignment, 
    IfStmt, WhileStmt, ForStmt, InfiniteStmt, BreakStmt, ContinueStmt
};
use crate::tests_consts::{
    ALL_TYPES_NO_ARR, ALL_TYPES_NO_ARR_SCATTERED, ALL_TYPES_NO_ARR_NO_USIZE, ALL_TYPES_NO_INTS_NO_ARR,
 
    ALL_TYPES_NO_ARR_NO_BOOL,
    ALL_TYPES_NO_ARR_NO_BOOL_NO_STRING, ALL_TYPES_NO_ARR_NO_BOOL_NO_STRING_SCATTERED,

    ALL_INT_TYPES_NO_ARR,

    ALL_UNSIGNED_TYPES_NO_ARR, ALL_SIGNED_TYPES_NO_ARR,
    ALL_BIN_OP_KIND_ARTH, ALL_BIN_OP_KIND_COMP, ALL_BIN_OP_KIND_COMP_EQ,
    ALL_BIN_OP_KIND_REAL_ARTH, ALL_BIN_OP_KIND_BIT_ARTH,

    ALL_BIN_OP_KIND,
    ALL_BIN_OP_KIND_LOGIC,
    ALL_BIN_OP_KIND_COMP_ARTH
};

mod const_tests;
mod var_decl_tests;
mod var_multi_decl_tests;
mod var_assign_tests;
mod ownership_tests;
mod expr_tests;
mod copy_tests;
mod format_tests;

mod int_literals_internal_inference_tests;

mod return_tests;
mod multi_return_tests;

mod function_tests;
mod function_call_tests;

mod locking_unlocking_tests;

mod bin_op_tests;
mod unary_op_tests;

mod array_tests;
mod dyn_array_tests;
mod dyn_array_access_tests;
mod dyn_array_slicing_tests;

mod fixed_array_tests;
mod fixed_array_access_tests;
mod fixed_array_slicing_tests;

mod if_stmt_tests;

mod for_stmt_tests;
mod while_stmt_tests;
mod infinite_stmt_tests;
mod break_stmt_tests;
mod continue_stmt_tests;

mod happy_path_tests;


