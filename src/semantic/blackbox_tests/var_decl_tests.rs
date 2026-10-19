use super::*;

#[cfg(test)]
mod var_decl_tests {
    use super::*;

    // TODO: Add new test that hyper focuses on integers coericon, instead of this weak tests
    // I mean I already added such tests in int_internal_inference but idk.. maybe need more..
    //
    // TODO: Add check_call tests here.
    //

    #[test]
    fn literals() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![var_decl(true, "x", t.clone(), l.clone())];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                assert_eq!(v.type_name, t.clone());
                assert_eq!(v.value, l.clone());
            } else {
                panic!("expected VarDecl");
            }
        }
    }

        
    #[test]
    fn var_name_taken_by_same_func_errors() {
        for t in ALL_TYPES_NO_ARR {
            let main = void_func("main", vec![], vec![
                var_decl(true, "main", t.clone(), call_expr("foo", vec![])),
            ]);

            let mut ast = AST { functions: vec![main], globals: vec![] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());        
            assert!(result.unwrap_err().to_string().contains("is already taken by a function"));
        }
    }

    #[test]
    fn var_name_taken_by_different_func_errors() {
        for t in ALL_TYPES_NO_ARR {
            let main = void_func("main", vec![], vec![
                var_decl(true, "foo", t.clone(), call_expr("foo", vec![])),
            ]);

            let foo = void_func("foo", vec![], vec![]);

            let mut ast = AST { functions: vec![main, foo], globals: vec![] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());        
            assert!(result.unwrap_err().to_string().contains("is already taken by a function"));
        }
    }

    #[test]
    fn non_declared_var_as_value_errors() {
        for t in ALL_TYPES_NO_ARR {
            let body = vec![var_decl(true, "x", t.clone(), var_expr("y"))];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Use of undeclared binding `y`"));
        }
    }


    #[test]
    fn var_decl_type_mismatch_errors() {
        let literals_no_ints = get_all_literals_no_arr_no_ints();

        for t in ALL_INT_TYPES_NO_ARR {
            for l in &literals_no_ints {
                let body = vec![var_decl(true, "x", t.clone(), l.clone())];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);

                assert!(result.is_err());        
                assert!(result.unwrap_err().to_string().contains("Type mismatch assigning to"));
            }
        }
    }


    // Tests the rule: 
    // You cannot overshadow variables declared in an upstream scope
    //

    #[test]
    fn vardecl_overshadowing_upstream_var_in_for_loop_holder_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let arr_lit = array_lit(vec![], Some(t.clone()));

            let body = vec![
                var_decl(true, "x", t.clone(), l.clone()),
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
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("is already declared"));
        }
    }



    #[test]
    fn vardecl_overshadowing_var_in_for_loop_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let arr_lit = array_lit(vec![], Some(t.clone()));

            let body = vec![
                var_decl(true, "x", t.clone(), l.clone()),
                var_decl(true, "a", Type::Array(Box::new(t.clone())), arr_lit),
                Stmt::For(ForStmt{
                        holder_name: "e".to_string(),
                        value: var_expr("a"),
                        branch: vec![
                            var_decl(true, "x", t.clone(), l.clone())
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
    fn vardecl_overshadowing_var_in_while_loop_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl(true, "x", t.clone(), l.clone()),
                Stmt::While(WhileStmt{
                        condition: bool_lit(false),
                        branch: vec![
                            var_decl(true, "x", t.clone(), l.clone())
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
    fn vardecl_overshadowing_var_in_infinite_loop_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl(true, "x", t.clone(), l.clone()),
                Stmt::Infinite(InfiniteStmt{
                        branch: vec![
                            var_decl(true, "x", t.clone(), l.clone())
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
    fn vardecl_overshadowing_var_in_if_main_branch_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl(true, "x", t.clone(), l.clone()),
                Stmt::If(IfStmt{
                    condition: bool_lit(false),
                    if_branch: vec![
                        var_decl(true, "x", t.clone(), l.clone())
                    ],
                    elif_branches: vec![],
                    else_branch: None,
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
    fn vardecl_overshadowing_var_in_if_else_branch_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl(true, "x", t.clone(), l.clone()),
                Stmt::If(IfStmt{
                    condition: bool_lit(false),
                    if_branch: vec![
                        // Just dummy declaration, so we don't get flagged by dead code because
                        // of empty branch.
                        var_decl(true, "z", t.clone(), l.clone()),

                    ],
                    elif_branches: vec![],
                    else_branch: Some(vec![
                        var_decl(true, "x", t.clone(), l.clone())
                    ]),
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
    fn vardecl_overshadowing_var_in_if_elif_branch_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl(true, "x", t.clone(), l.clone()),
                Stmt::If(IfStmt{
                    condition: bool_lit(false),
                    if_branch: vec![
                        // Just dummy declaration, so we don't get flagged by dead code because
                        // of empty branch.
                        var_decl(true, "z", t.clone(), l.clone()),
                    ],
                    elif_branches: vec![
                        (bool_lit(false), vec![
                            var_decl(true, "x", t.clone(), l.clone())
                        ])
                    ],
                    else_branch: None,
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

    // This tests  integers / floats only, against Bool / String
    #[test]
    fn vardecl_type_mismatch_int_bool_errors() {

        let literals_ints_floats = get_all_literals_no_arr_str_bool();

        for l in &literals_ints_floats {
            // Variables declared with explicit type of bool, but given an non-bool literal is a type mismatch
            let body = vec![var_decl(true, "x", Type::Bool, l.clone())];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());    
            assert!(result.unwrap_err().to_string().contains("Type mismatch assigning to"));
        }


        for l in literals_ints_floats {
            // Variables declared with explicit type of string, but given an non-string literal is a type mismatch
            let body = vec![var_decl(true, "x", Type::String, l.clone())];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Type mismatch assigning to"));
        }

    }

    #[test]
    fn use_of_undeclared_variable_other_errors() {
        // Try referencing non-existent variable "y"
        for t in ALL_TYPES_NO_ARR {
            let body = vec![var_decl(true, "x", t.clone(), var_expr("y"))]; // y not declared
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("undeclared binding"));
        }
    }

    #[test]
    fn use_of_undeclared_variable_ourself_errors() {
        // Try referencing non-existent variable "x" aka ourselves.
        for t in ALL_TYPES_NO_ARR {
            let body = vec![var_decl(true, "x", t.clone(), var_expr("x"))]; // x not declared
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("undeclared binding"));
        }
    }



}

