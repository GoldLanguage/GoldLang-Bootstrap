use super::*;

#[cfg(test)]
mod return_tests {
    use super::*;

    // Code after return is not allowed
    //
    #[test]
    fn code_after_return_errors() {
        let literals = get_all_literals();

        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            let body = vec![
                return_stmt(vec![l.clone()]),
                var_decl(true, "x", t.clone(), l.clone()),
            ];
            let func = returning_func("foo", vec![], vec![t.clone()], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Unreachable statement"));
        }
    }

    // Missing return statement in a function with return types
    //
    #[test]
    fn missing_return_in_returning_func_errors() {
        let literals = get_all_literals();

        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            let body = vec![var_decl(true, "x", t.clone(), l.clone())];
            let func = returning_func("foo", vec![], vec![t.clone()], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Expected function `foo` to return, but we found no return statements"))
        }
    }

    #[test]
    fn return_in_void_func_errors() {
        let literals = get_all_literals();
        
        for l in literals {
            // Void function that tries to return a value.
            let body = vec![return_stmt(vec![l.clone()])];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("no declared return type"));
        }
    }


    #[test]
    fn void_func_in_expr_errors() {
        for t in ALL_TYPES_WITH_DYN_ARR.iter() {
            let callee = void_func("bar", vec![], vec![]);
            let body = vec![
                var_decl(true, "x", t.clone(), call_expr("bar", vec![]))
            ];
            let caller = void_func("main", vec![], body);
            let mut ast = AST { functions: vec![callee, caller], globals: vec![] };

            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("has no return type declared but is used in an expression"));
        }
    }


    #[test]
    fn type_mismatch_return_errors() {
        let literals = get_all_literals_few_ints();

        for (l, t) in literals.iter().zip(ALL_TYPES_FEW_INTS_WITH_DYN_ARR_SCATTERED.iter()) {
            let body = vec![return_stmt(vec![l.clone()])];
            let func = returning_func("foo", vec![], vec![t.clone()], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            let res_str = result.unwrap_err().to_string();

            assert!(res_str.contains("Return type mismatch") || res_str.contains("out of range"));
        }
    }

    #[test]
    fn return_count_mismatch_errors() {
        // Declares two return types but returns one value.

        let literals = get_all_literals();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            let body = vec![return_stmt(vec![l.clone()])];
            let func = returning_func("foo", vec![], vec![t.clone(), t.clone()], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Return length mismatch"));
        }
    }



}

