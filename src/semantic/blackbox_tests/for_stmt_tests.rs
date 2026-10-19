use super::*;

#[cfg(test)]
mod for_stmt_tests {
    use super::*;

    #[test]
    fn for_stmt_value_is_var() {
        let literals = get_all_literals();

        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for i in 0..=100 {
                let elements = vec![l.clone(); i];

                let arr_lit = array_lit(elements.clone(), Some(t.clone()));

                let body = vec![ 
                    var_decl(true, "a", Type::Array(Box::new(t.clone())), arr_lit),
                    Stmt::For(ForStmt{
                        holder_name: "x".to_string(),
                        value: var_expr("a"),
                        branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl(true, "z", t.clone(), l.clone()),
                        ],
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);

                assert!(result.is_ok())
            }
        }
    }

    #[test]
    fn for_stmt_value_is_array_literal() {
        let literals = get_all_literals();

        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for i in 0..=100 {
                let elements = vec![l.clone(); i];

                let arr_lit = array_lit(elements.clone(), Some(t.clone()));

                let body = vec![ 
                    Stmt::For(ForStmt{
                        holder_name: "x".to_string(),
                        value: arr_lit,
                        // Just dummy declaration, so we don't get flagged by dead code because of empty branch.
                        branch: vec![var_decl(true, "z", t.clone(), l.clone())],
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);

                assert!(result.is_ok())
            }
        }
    }

    #[test]
    fn for_stmt_with_fixed_arrays() {
        let literals = get_all_literals();

        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for i in 0..=100 {
                let elements = vec![l.clone(); i];

                let arr_lit = array_lit(elements.clone(), Some(t.clone()));

                let body = vec![ 
                    var_decl(true, "a", Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i)), arr_lit),
                    Stmt::For(ForStmt{
                        holder_name: "x".to_string(),
                        value: var_expr("a"),
                        branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl(true, "z", t.clone(), l.clone()),
                        ],
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);

                assert!(result.is_ok())
            }
        }
    }


    // Test for statements with rangecall, with only integer literals, no variables.
    #[test]
    fn for_stmt_with_range_int_literals() {
        let literals = get_all_literals_no_arr_str_bool_float();

        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![ 
                Stmt::For(ForStmt{
                    holder_name: "x".to_string(),
                    value: Expr::RangeCall{
                        start: Box::new(l.clone()),
                        end: Box::new(l.clone()),
                        span: span()
                    },
                    
                    branch: vec![
                        // Just dummy declaration, so we don't get flagged by dead code because
                        // of empty branch.
                        var_decl(true, "z", t.clone(), l.clone()),
                    ],
                    span: span(),
                }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);

            assert!(result.is_ok())
        }
    }


    #[test]
    fn for_stmt_with_range_non_int_literals_errors() {
        let literals_no_ints = get_all_literals_no_arr_no_ints();

        for (l, t) in literals_no_ints.iter().zip(ALL_TYPES_NO_INTS_NO_ARR.iter()) {
            let body = vec![ 
                Stmt::For(ForStmt{
                    holder_name: "x".to_string(),
                    value: Expr::RangeCall{
                        start: Box::new(l.clone()),
                        end: Box::new(l.clone()),
                        span: span()
                    },
                    
                    branch: vec![
                        // Just dummy declaration, so we don't get flagged by dead code because
                        // of empty branch.
                        var_decl(true, "z", t.clone(), l.clone()),
                    ],
                    span: span(),
                }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Expected range arguments to be any Integer type"));
        }
    }


    #[test]
    fn for_stmt_with_range_mixed_literals_errors() {
        let literals_no_ints = get_all_literals_no_arr_no_ints();
        let literals = get_all_literals_no_arr();

        for ((l, t), l2) in literals_no_ints.iter()
            .zip(ALL_TYPES_NO_INTS_NO_ARR.iter())
            .zip(literals.iter())
        {
            let body = vec![ 
                Stmt::For(ForStmt{
                    holder_name: "x".to_string(),
                    value: Expr::RangeCall{
                        start: Box::new(l.clone()),
                        end: Box::new(l2.clone()),
                        span: span()
                    },
                    
                    branch: vec![
                        // Just dummy declaration, so we don't get flagged by dead code because
                        // of empty branch.
                        var_decl(true, "z", t.clone(), l.clone()),
                    ],
                    span: span(),
                }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Expected range arguments to be of the same type"));
        }

        for ((l, t), l2) in literals_no_ints.iter()
            .zip(ALL_TYPES_NO_INTS_NO_ARR.iter())
            .zip(literals.iter())
        {
            let body = vec![ 
                Stmt::For(ForStmt{
                    holder_name: "x".to_string(),
                    value: Expr::RangeCall{
                        start: Box::new(l2.clone()),
                        end: Box::new(l.clone()),
                        span: span()
                    },
                    
                    branch: vec![
                        // Just dummy declaration, so we don't get flagged by dead code because
                        // of empty branch.
                        var_decl(true, "z", t.clone(), l.clone()),
                    ],
                    span: span(),
                }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Expected range arguments to be of the same type"));
        }

    }

    #[test]
    fn for_stmt_holder_name_already_taken_by_var_errors() {
        let literals = get_all_literals();

        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for i in 0..=100 {
                let elements = vec![l.clone(); i];

                let arr_lit = array_lit(elements.clone(), Some(t.clone()));

                let body = vec![ 
                    var_decl(true, "a", Type::Array(Box::new(t.clone())), arr_lit.clone()),
                    var_decl(true, "x", Type::Array(Box::new(t.clone())), arr_lit),
                    Stmt::For(ForStmt{
                        holder_name: "x".to_string(),
                        value: var_expr("a"),
                        branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl(true, "z", t.clone(), l.clone()),
                        ],
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);

                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("is already declared"));
            }
        }
    }

    #[test]
    fn for_stmt_holder_name_already_taken_by_func_errors() {
        let literals = get_all_literals();

        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for i in 0..=100 {
                let elements = vec![l.clone(); i];

                let arr_lit = array_lit(elements.clone(), Some(t.clone()));

                let body = vec![ 
                    var_decl(true, "a", Type::Array(Box::new(t.clone())), arr_lit.clone()),
                    Stmt::For(ForStmt{
                        holder_name: "pair".to_string(),
                        value: var_expr("a"),
                        branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl(true, "z", t.clone(), l.clone()),
                        ],
                        span: span(),
                    }),
                ];

                let pair_body = vec![return_stmt(vec![l.clone()])];
                let pair = returning_func("pair", vec![], vec![t.clone()], pair_body);

                let main = void_func("foo", vec![], body);
                let mut ast = AST { functions: vec![main, pair], globals: vec![] };
                let result = check_semantics(&mut ast);

                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("is already taken by a function"));
            }
        }
    }

    #[test]
    fn for_stmt_with_range_holder_name_is_already_taken_by_var_errors() {
        let literals = get_all_literals_no_arr_str_bool_float();

        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![ 
                var_decl(true, "x", t.clone(), l.clone()),
                Stmt::For(ForStmt{
                    holder_name: "x".to_string(),
                    value: Expr::RangeCall{
                        start: Box::new(l.clone()),
                        end: Box::new(l.clone()),
                        span: span()
                    },
                    
                    branch: vec![
                        // Just dummy declaration, so we don't get flagged by dead code because
                        // of empty branch.
                        var_decl(true, "z", t.clone(), l.clone()),
                    ],
                    span: span(),
                }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("is already declared"));
        }
    }

    #[test]
    fn for_stmt_with_range_holder_name_is_already_taken_by_func_errors() {
        let literals = get_all_literals_no_arr_str_bool_float();

        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![ 
                Stmt::For(ForStmt{
                    holder_name: "pair".to_string(),
                    value: Expr::RangeCall{
                        start: Box::new(l.clone()),
                        end: Box::new(l.clone()),
                        span: span()
                    },
                    
                    branch: vec![
                        // Just dummy declaration, so we don't get flagged by dead code because
                        // of empty branch.
                        var_decl(true, "z", t.clone(), l.clone()),
                    ],
                    span: span(),
                }),
            ];
            let pair_body = vec![return_stmt(vec![l.clone()])];
            let pair = returning_func("pair", vec![], vec![t.clone()], pair_body);

            let main = void_func("foo", vec![], body);
            let mut ast = AST { functions: vec![main, pair], globals: vec![] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("is already taken by a function"));
        }
    }


    #[test]
    fn for_stmt_with_fixed_array_holder_name_is_already_taken_by_var_errors() {
        let literals = get_all_literals();

        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for i in 0..=100 {
                let elements = vec![l.clone(); i];

                let arr_lit = array_lit(elements.clone(), Some(t.clone()));

                let body = vec![ 
                    var_decl(true, "a", Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i)), arr_lit),
                    var_decl(true, "x", t.clone(), l.clone()),
                    Stmt::For(ForStmt{
                        holder_name: "x".to_string(),
                        value: var_expr("a"),

                        branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl(true, "z", t.clone(), l.clone()),
                        ],
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);

                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("is already declared"));
            }
        }
    }

    #[test]
    fn for_stmt_with_no_array_no_range() {
        let literals = get_all_literals_no_arr();

        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter())
        {
            let body = vec![ 
                Stmt::For(ForStmt{
                    holder_name: "x".to_string(),
                    value: l.clone(),
                    branch: vec![
                        // Just dummy declaration, so we don't get flagged by dead code because
                        // of empty branch.
                        var_decl(true, "z", t.clone(), l.clone()),
                    ],
                    span: span(),
                }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("For loop statement require an expression to be evaulatable to any `Array` type"));
        }
    }


    #[test]
    fn for_stmt_fixed_arr_empty_branch_errors() {
        let literals = get_all_literals();

        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for i in 0..=100 {
                let elements = vec![l.clone(); i];

                let arr_lit = array_lit(elements.clone(), Some(t.clone()));

                let body = vec![ 
                    var_decl(true, "a", Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i)), arr_lit),
                    Stmt::For(ForStmt{
                        holder_name: "x".to_string(),
                        value: var_expr("a"),

                        branch: vec![],
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);

                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("For loop branch has no statements"));
            }
        }
    }

    #[test]
    fn for_stmt_dyn_arr_empty_branch_errors() {
        let literals = get_all_literals_no_arr();

        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 0..=100 {
                let elements = vec![l.clone(); i];

                let arr_lit = array_lit(elements.clone(), Some(t.clone()));

                let body = vec![ 
                    var_decl(true, "a", Type::Array(Box::new(t.clone())), arr_lit),
                    Stmt::For(ForStmt{
                        holder_name: "x".to_string(),
                        value: var_expr("a"),

                        branch: vec![],
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);

                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("For loop branch has no statements"));
            }
        }
    }


}
