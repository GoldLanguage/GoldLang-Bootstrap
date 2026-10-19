use super::*;

#[cfg(test)]
mod var_multi_decl_tests {
    use super::*;

    #[test]
    fn unknown_func_errors() {
        for t in ALL_TYPES_WITH_DYN_ARR.iter() {
            let vars = vec![MultiVariableDeclaration { name: "a".to_string(), type_name: t.clone(), span: span() }];
            let body = vec![Stmt::VarDeclMulti(vars, call_expr("pair", vec![]))];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![main], globals: vec![] };
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("unknown function"));
        }
    }

    #[test]
    fn var_name_taken_by_same_func_errors() {
        let literals = get_all_literals();

        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            let pair_body = vec![return_stmt(vec![l.clone()])];
            let pair = returning_func("pair", vec![], vec![t.clone()], pair_body);

            let vars = vec![
                MultiVariableDeclaration { name: "main".to_string(), type_name: t.clone(), span: span() },
            ];
            let body = vec![Stmt::VarDeclMulti(vars, call_expr("pair", vec![]))];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![main, pair], globals: vec![] };
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("is already taken by a function"));
        }
    }

    #[test]
    fn var_name_taken_by_different_func_errors() {
        let literals = get_all_literals();

        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            let pair_body = vec![return_stmt(vec![l.clone()])];
            let pair = returning_func("pair", vec![], vec![t.clone()], pair_body);

            let vars = vec![
                MultiVariableDeclaration { name: "pair".to_string(), type_name: t.clone(), span: span() },
            ];
            let body = vec![Stmt::VarDeclMulti(vars, call_expr("pair", vec![]))];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![main, pair], globals: vec![] };
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("is already taken by a function"));
        }
    }

    #[test]
    fn unknown_func_call_in_func_call_arg_errors() {
        let literals = get_all_literals();

        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for i in 0..=100 {
                let pair_body = vec![return_stmt(vec![l.clone()])];
                let pair = returning_func("pair", vec![param("x", t.clone())], vec![t.clone()], pair_body);

                let vars = vec![
                    MultiVariableDeclaration { name: "a".to_string(), type_name: t.clone(), span: span() },
                ];
                let body = vec![Stmt::VarDeclMulti(vars, call_expr("pair", vec![call_expr("hi", vec![l.clone();i])]))];
                let main = void_func("main", vec![], body);

                let mut ast = AST { functions: vec![main, pair], globals: vec![] };
                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("unknown function"));
            }
        }
    }

    #[test]
    fn type_mismatch_in_func_call_args_errors() {
        let literals = get_all_literals_few_ints();
        let literals_scattered = get_all_literals_few_ints_scattered();


        for (((l1, t1), l2), t2) in literals.iter()
                .zip(ALL_TYPES_FEW_INTS_WITH_DYN_ARR.iter())
                .zip(literals_scattered.iter())
                .zip(ALL_TYPES_FEW_INTS_WITH_DYN_ARR_SCATTERED.iter()) 
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l2.clone()])];
            let pair = returning_func("pair", vec![param("x", t1.clone()), param("y", t2.clone())], vec![t1.clone(), t2.clone()], pair_body);

            let vars = vec![
                MultiVariableDeclaration { name: "a".to_string(), type_name: t1.clone(), span: span() },
                MultiVariableDeclaration { name: "b".to_string(), type_name: t2.clone(), span: span() }
            ];

            let body = vec![Stmt::VarDeclMulti(vars, call_expr("pair", vec![l2.clone(), l1.clone()]))];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![main, pair], globals: vec![] };
            let result = check_semantics(&mut ast);
            assert!(result.is_err());

            let err_str = result.unwrap_err().to_string();

            assert!(err_str.contains("type mismatch") || (err_str.contains("Integer literal") && err_str.contains("out of range")) );
        }
    }



    #[test]
    fn wrong_arg_arity_errors() {
        let literals = get_all_literals_no_arr();
        let literals_scattered = get_all_literals_no_arr_scattered_order();

        for (((l1, t1), l2), t2) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()).zip(literals_scattered.iter()).zip(ALL_TYPES_NO_ARR_SCATTERED) {
            for i in 2..=1002 {
                let pair_body = vec![return_stmt(vec![l1.clone(), l2.clone()])];
                let pair = returning_func("pair", vec![param("x", t1.clone())], vec![t1.clone(), t2.clone()], pair_body);

                let vars = vec![
                    MultiVariableDeclaration { name: "a".to_string(), type_name: t1.clone(), span: span() },
                    MultiVariableDeclaration { name: "b".to_string(), type_name: t2.clone(), span: span() }
                ];
                let body = vec![Stmt::VarDeclMulti(vars, call_expr("pair", vec![l1.clone();i]))];
                let main = void_func("main", vec![], body);

                let mut ast = AST { functions: vec![main, pair], globals: vec![] };
                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("expects 1 argument"));
            }
        }
    }
}

