use super::*;

#[cfg(test)]
mod empty_branch_analysis_tests {
    use super::*;

    #[test]
    #[should_panic(expected="Compiler bug")]
    // The reason `empty_branch_analysis_hazmat` panics when fed empty block of code directly, is
    // because it expects caller to give it an initial, non empty block of code. because if it were
    // given empty block of code, the function wouldn't be able to print error with line and column, and i dont want keep
    // passing spans all over.
    //
    fn empty_block_of_code_panics() {
        let _ = empty_branch_analysis_hazmat(&vec![]);
    }

    #[test]
    fn empty_infinite_stmt_errors() {
        for i in 1..1000 {
            let result = empty_branch_analysis_hazmat(&vec![Stmt::Infinite(InfiniteStmt{ branch: vec![], span: span() }); i]);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Infinite loop branch has no statements"));
        }
    }

    #[test]
    fn empty_nested_infinite_stmt_errors() {
        let mut stmt = Stmt::Infinite(InfiniteStmt{ branch: vec![], span: span() });

        for _ in 1..=100 {
            stmt = Stmt::Infinite(InfiniteStmt{ branch: vec![stmt], span: span() });

            let result = empty_branch_analysis_hazmat(&vec![stmt.clone()]);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Infinite loop branch has no statements"));
        }
    }

    #[test]
    fn empty_infinite_stmt_inside_while_stmt_errors() {
        let literals = get_all_literals();
        for l in literals {
            let mut stmt = Stmt::While(WhileStmt{ condition: l.clone(), branch: vec![
                Stmt::Infinite(InfiniteStmt{ branch: vec![], span: span() })
            ], span: span() });

            for _ in 1..=100 {
                stmt = Stmt::While(WhileStmt{ condition: l.clone(), branch: vec![stmt], span: span() });

                let result = empty_branch_analysis_hazmat(&vec![stmt.clone()]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Infinite loop branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_infinite_stmt_inside_for_stmt_errors() {
        let literals = get_all_literals();
        for l in literals {
            let mut stmt = Stmt::For(ForStmt{ holder_name: "x".to_string(), value: l.clone(), branch: vec![
                Stmt::Infinite(InfiniteStmt{ branch: vec![], span: span() })
            ], span: span() });

            for _ in 1..=100 {
                stmt = Stmt::For(ForStmt{ holder_name: "x".to_string(), value: var_expr("a"), branch: vec![stmt], span: span() });

                let result = empty_branch_analysis_hazmat(&vec![stmt.clone()]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Infinite loop branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_infinite_stmt_in_if_stmt_main_branch_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
                let stmt = Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![Stmt::Infinite(InfiniteStmt{ branch: vec![], span: span()})],
                        elif_branches: vec![],
                        else_branch: None,
                        span: span()
                });

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Infinite loop branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_infinite_stmt_in_if_stmt_else_branch_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
                let stmt = Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ Stmt::Expr(l.clone()) ], // dummy statement
                        elif_branches: vec![],
                        else_branch: Some(vec![Stmt::Infinite(InfiniteStmt{ branch: vec![], span: span()})]),
                        span: span() });

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Infinite loop branch has no statements"));
            }
        }
    }


    #[test]
    fn empty_infinite_stmt_in_if_stmt_elif_branch_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
                let stmt = Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ Stmt::Expr(l.clone()) ], // dummy statement
                        elif_branches: vec![(l.clone(), vec![Stmt::Infinite(InfiniteStmt{ branch: vec![], span: span()})]); i],
                        else_branch: None,
                        span: span() });

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Infinite loop branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_infinite_stmt_in_if_stmt_main_and_else_branch_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
            let stmt = Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![Stmt::Infinite(InfiniteStmt{ branch: vec![], span: span()})],
                        elif_branches: vec![],
                        else_branch: Some(vec![Stmt::Infinite(InfiniteStmt{ branch: vec![], span: span()})]),
                        span: span() });

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Infinite loop branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_infinite_stmt_if_stmt_main_and_elif_branch_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
                let stmt = Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ Stmt::Infinite(InfiniteStmt{ branch: vec![], span: span()}) ],
                        elif_branches: vec![(l.clone(), vec![ Stmt::Infinite(InfiniteStmt{ branch: vec![], span: span()}) ]); i],
                        else_branch: None,
                        span: span() });

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Infinite loop branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_infinite_stmt_in_if_stmt_elif_and_else_branch_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
                let stmt = Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ Stmt::Expr(l.clone()) ], // dummy statement
                        elif_branches: vec![(l.clone(), vec![ Stmt::Infinite(InfiniteStmt{ branch: vec![], span: span()}) ])],
                        else_branch: Some(vec![ Stmt::Infinite(InfiniteStmt{ branch: vec![], span: span()}) ]),
                        span: span() });

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Infinite loop branch has no statements"));
            }
        }
    }


    #[test]
    fn empty_nested_infinite_stmt_in_if_stmt_main_branch_errors() {
        let literals = get_all_literals();
        for l in literals {
            let mut stmt = Stmt::If(IfStmt{ condition: l.clone(), if_branch: vec![
                Stmt::Infinite(InfiniteStmt{ branch: vec![], span: span()})
                ], elif_branches: vec![], else_branch: None, span: span()});

            for _ in 1..=100 {
                stmt = Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ Stmt::Infinite(InfiniteStmt{ branch: vec![ stmt ], span: span()}) ],
                        elif_branches: vec![],
                        else_branch: None,
                    span: span() });

                let result = empty_branch_analysis_hazmat(&vec![stmt.clone()]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Infinite loop branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_nested_infinite_stmt_in_if_stmt_else_branch_errors() {
        let literals = get_all_literals();
        for l in literals {
            let mut stmt = Stmt::If(IfStmt{ condition: l.clone(), if_branch: vec![ Stmt::Expr(l.clone())], elif_branches: vec![], else_branch: Some(vec![
                Stmt::Infinite(InfiniteStmt{ branch: vec![], span: span()})
            ]), span: span()});

            for _ in 1..=100 {
                stmt = Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ Stmt::Expr(l.clone()) ],
                        elif_branches: vec![],
                        else_branch: Some(vec![ stmt ]),
                        span: span() });

                let result = empty_branch_analysis_hazmat(&vec![stmt.clone()]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Infinite loop branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_nested_infinite_stmt_in_if_stmt_elif_branch_errors() {
        let literals = get_all_literals();
        for l in literals {
            let mut stmt = Stmt::If(IfStmt{ condition: l.clone(), if_branch: vec![ Stmt::Expr(l.clone())], elif_branches: vec![], else_branch: Some(vec![
                Stmt::Infinite(InfiniteStmt{ branch: vec![], span: span()})
            ]), span: span()});

            for _ in 1..=100 {
                stmt = Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ Stmt::Expr(l.clone()) ],
                        elif_branches: vec![(l.clone(), vec![stmt])],
                        else_branch: None,
                        span: span() });

                let result = empty_branch_analysis_hazmat(&vec![stmt.clone()]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Infinite loop branch has no statements"));
            }
        }
    }


    // Same tests as above, except this time it's for While loop statements
    //
    #[test]
    fn empty_while_stmt_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
                let result = empty_branch_analysis_hazmat(&vec![Stmt::While(WhileStmt{ condition: l.clone(), branch: vec![], span: span() }); i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("While loop branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_nested_while_stmt_errors() {
        let literals = get_all_literals();
        for l in literals {
            let mut stmt = Stmt::While(WhileStmt{ condition: l.clone(), branch: vec![], span: span() });

            for _ in 1..=100 {
                stmt = Stmt::While(WhileStmt{ condition: l.clone(), branch: vec![stmt], span: span() });

                let result = empty_branch_analysis_hazmat(&vec![stmt.clone()]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("While loop branch has no statements"));
            }
        }
    }


    #[test]
    fn empty_while_stmt_inside_infinite_stmt_errors() {
        let literals = get_all_literals();
        for l in literals {
            let mut stmt = Stmt::Infinite(InfiniteStmt{ branch: vec![
                    Stmt::While(WhileStmt{ condition: l.clone(), branch: vec![], span: span() })
                ], span: span() });

            for _ in 1..=100 {
                stmt = Stmt::Infinite(InfiniteStmt{ branch: vec![stmt], span: span() });

                let result = empty_branch_analysis_hazmat(&vec![stmt.clone()]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("While loop branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_while_stmt_inside_for_stmt_errors() {
        let literals = get_all_literals();
        for l in literals {
            let mut stmt = Stmt::For(ForStmt{ holder_name: "x".to_string(), value: l.clone(), branch: vec![
                    Stmt::While(WhileStmt{ condition: l.clone(), branch: vec![], span: span() })
                ], span: span() });

            for _ in 1..=100 {
                stmt = Stmt::For(ForStmt{ holder_name: "x".to_string(), value: l.clone(), branch: vec![stmt], span: span() });

                let result = empty_branch_analysis_hazmat(&vec![stmt.clone()]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("While loop branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_while_stmt_in_if_stmt_main_branch_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
                let stmt = Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![Stmt::While(WhileStmt{ condition: l.clone(), branch: vec![], span: span() })],
                        elif_branches: vec![],
                        else_branch: None,
                        span: span()
                });

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("While loop branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_while_stmt_in_if_stmt_else_branch_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
                let stmt = Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ Stmt::Expr(l.clone()) ], // dummy statement
                        elif_branches: vec![],
                        else_branch: Some(vec![Stmt::While(WhileStmt{ condition: l.clone(), branch: vec![], span: span() })]),
                        span: span() });

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("While loop branch has no statements"));
            }
        }
    }


    #[test]
    fn empty_while_stmt_in_if_stmt_elif_branch_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
                let stmt = Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ Stmt::Expr(l.clone()) ], // dummy statement
                        elif_branches: vec![(l.clone(), vec![Stmt::While(WhileStmt{ condition: l.clone(), branch: vec![], span: span() })])],
                        else_branch: None,
                        span: span() });

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("While loop branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_while_stmt_in_if_stmt_main_and_else_branch_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
            let stmt = Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![Stmt::While(WhileStmt{ condition: l.clone(), branch: vec![], span: span() })],
                        elif_branches: vec![],
                        else_branch: Some(vec![Stmt::While(WhileStmt{ condition: l.clone(), branch: vec![], span: span() })]),
                        span: span() });

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("While loop branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_while_stmt_if_stmt_main_and_elif_branch_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
                let stmt = Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![Stmt::While(WhileStmt{ condition: l.clone(), branch: vec![], span: span() })],
                        elif_branches: vec![(l.clone(), vec![ Stmt::While(WhileStmt{ condition: l.clone(), branch: vec![], span: span() })])],
                        else_branch: None,
                        span: span() });

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("While loop branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_while_stmt_in_if_stmt_elif_and_else_branch_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
                let stmt = Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ Stmt::Expr(l.clone()) ], // dummy statement
                        elif_branches: vec![(l.clone(), vec![ Stmt::While(WhileStmt{ condition: l.clone(), branch: vec![], span: span() }) ])],
                        else_branch: Some(vec![ Stmt::While(WhileStmt{ condition: l.clone(), branch: vec![], span: span() })]),
                        span: span() });

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("While loop branch has no statements"));
            }
        }
    }


    #[test]
    fn empty_nested_while_stmt_in_if_stmt_main_branch_errors() {
        let literals = get_all_literals();
        for l in literals {
            let mut stmt = Stmt::If(IfStmt{ condition: l.clone(), if_branch: vec![
                Stmt::While(WhileStmt{ condition: l.clone(), branch: vec![], span: span() })
                ], elif_branches: vec![], else_branch: None, span: span()});

            for _ in 1..=100 {
                stmt = Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![Stmt::While(WhileStmt{ condition: l.clone(), branch: vec![ stmt ], span: span() })],
                        elif_branches: vec![],
                        else_branch: None,
                    span: span() });

                let result = empty_branch_analysis_hazmat(&vec![stmt.clone()]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("While loop branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_nested_while_stmt_in_if_stmt_else_branch_errors() {
        let literals = get_all_literals();
        for l in literals {
            let mut stmt = Stmt::If(IfStmt{ condition: l.clone(), if_branch: vec![ Stmt::Expr(l.clone())], elif_branches: vec![], else_branch: Some(vec![
                Stmt::While(WhileStmt{ condition: l.clone(), branch: vec![], span: span() })
            ]), span: span()});

            for _ in 1..=100 {
                stmt = Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ Stmt::Expr(l.clone()) ],
                        elif_branches: vec![],
                        else_branch: Some(vec![ stmt ]),
                        span: span() });

                let result = empty_branch_analysis_hazmat(&vec![stmt.clone()]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("While loop branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_nested_while_stmt_in_if_stmt_elif_branch_errors() {
        let literals = get_all_literals();
        for l in literals {
            let mut stmt = Stmt::If(IfStmt{ condition: l.clone(), if_branch: vec![ Stmt::Expr(l.clone())], elif_branches: vec![], else_branch: Some(vec![
                Stmt::While(WhileStmt{ condition: l.clone(), branch: vec![], span: span() })
            ]), span: span()});

            for _ in 1..=100 {
                stmt = Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ Stmt::Expr(l.clone()) ],
                        elif_branches: vec![(l.clone(), vec![stmt])],
                        else_branch: None,
                        span: span() });

                let result = empty_branch_analysis_hazmat(&vec![stmt.clone()]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("While loop branch has no statements"));
            }
        }
    }




    // Same tests as above, except this time it's for For loop statements
    //
    #[test]
    fn empty_for_stmt_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
                let result = empty_branch_analysis_hazmat(&vec![Stmt::For(ForStmt{ holder_name: "x".to_string(), value: l.clone(), branch: vec![], span: span() }); i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("For loop branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_nested_for_stmt_errors() {
        let literals = get_all_literals();
        for l in literals {
            let mut stmt = Stmt::For(ForStmt{ holder_name: "x".to_string(), value: l.clone(), branch: vec![], span: span() });

            for _ in 1..=100 {
                stmt = Stmt::For(ForStmt{ holder_name: "x".to_string(), value: l.clone(), branch: vec![stmt], span: span() });

                let result = empty_branch_analysis_hazmat(&vec![stmt.clone()]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("For loop branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_for_stmt_inside_infinite_stmt_errors() {
        let literals = get_all_literals();
        for l in literals {
            let mut stmt = Stmt::Infinite(InfiniteStmt{ branch: vec![
                    Stmt::For(ForStmt{ holder_name: "x".to_string(), value: l.clone(), branch: vec![], span: span() })
                ], span: span() });

            for _ in 1..=100 {
                stmt = Stmt::Infinite(InfiniteStmt{ branch: vec![stmt], span: span() });

                let result = empty_branch_analysis_hazmat(&vec![stmt.clone()]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("For loop branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_for_stmt_inside_while_stmt_errors() {
        let literals = get_all_literals();
        for l in literals {
            let mut stmt = Stmt::While(WhileStmt{ condition: l.clone(), branch: vec![
                Stmt::For(ForStmt{ holder_name: "x".to_string(), value: l.clone(), branch: vec![], span: span() })
            ], span: span() });

            for _ in 1..=100 {
                stmt = Stmt::While(WhileStmt{ condition: l.clone(), branch: vec![stmt], span: span() });

                let result = empty_branch_analysis_hazmat(&vec![stmt.clone()]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("For loop branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_for_stmt_in_if_stmt_main_branch_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
                let stmt = Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![Stmt::For(ForStmt{ holder_name: "x".to_string(), value: l.clone(), branch: vec![], span: span() })],
                        elif_branches: vec![],
                        else_branch: None,
                        span: span()
                });

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("For loop branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_for_stmt_in_if_stmt_else_branch_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
                let stmt = Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ Stmt::Expr(l.clone()) ], // dummy statement
                        elif_branches: vec![],
                        else_branch: Some(vec![Stmt::For(ForStmt{ holder_name: "x".to_string(), value: l.clone(), branch: vec![], span: span() })]),
                        span: span() });

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("For loop branch has no statements"));
            }
        }
    }


    #[test]
    fn empty_for_stmt_in_if_stmt_elif_branch_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
                let stmt = Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ Stmt::Expr(l.clone()) ], // dummy statement
                        elif_branches: vec![(l.clone(), vec![Stmt::For(ForStmt{ holder_name: "x".to_string(), value: l.clone(), branch: vec![], span: span() })])],
                        else_branch: None,
                        span: span() });

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("For loop branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_for_stmt_in_if_stmt_main_and_else_branch_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
            let stmt = Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![Stmt::For(ForStmt{ holder_name: "x".to_string(), value: l.clone(), branch: vec![], span: span() })],
                        elif_branches: vec![],
                        else_branch: Some(vec![Stmt::For(ForStmt{ holder_name: "x".to_string(), value: l.clone(), branch: vec![], span: span() })]),
                        span: span() });

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("For loop branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_for_stmt_if_stmt_main_and_elif_branch_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
                let stmt = Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![Stmt::For(ForStmt{ holder_name: "x".to_string(), value: l.clone(), branch: vec![], span: span() })],
                        elif_branches: vec![(l.clone(), vec![ Stmt::For(ForStmt{ holder_name: "x".to_string(), value: l.clone(), branch: vec![], span: span() })])],
                        else_branch: None,
                        span: span() });

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("For loop branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_for_stmt_in_if_stmt_elif_and_else_branch_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
                let stmt = Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ Stmt::Expr(l.clone()) ], // dummy statement
                        elif_branches: vec![(l.clone(), vec![ Stmt::For(ForStmt{ holder_name: "x".to_string(), value: l.clone(), branch: vec![], span: span() })])],
                        else_branch: Some(vec![ Stmt::For(ForStmt{ holder_name: "x".to_string(), value: l.clone(), branch: vec![], span: span() })]),
                        span: span() });

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("For loop branch has no statements"));
            }
        }
    }


    #[test]
    fn empty_nested_for_stmt_in_if_stmt_main_branch_errors() {
        let literals = get_all_literals();
        for l in literals {
            let mut stmt = Stmt::If(IfStmt{ condition: l.clone(), if_branch: vec![
                Stmt::For(ForStmt{ holder_name: "x".to_string(), value: l.clone(), branch: vec![], span: span() })
                ], elif_branches: vec![], else_branch: None, span: span()});

            for _ in 1..=100 {
                stmt = Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![Stmt::For(ForStmt{ holder_name: "x".to_string(), value: l.clone(), branch: vec![ stmt ], span: span() })],
                        elif_branches: vec![],
                        else_branch: None,
                    span: span() });

                let result = empty_branch_analysis_hazmat(&vec![stmt.clone()]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("For loop branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_nested_for_stmt_in_if_stmt_else_branch_errors() {
        let literals = get_all_literals();
        for l in literals {
            let mut stmt = Stmt::If(IfStmt{ condition: l.clone(), if_branch: vec![ Stmt::Expr(l.clone())], elif_branches: vec![], else_branch: Some(vec![
                Stmt::For(ForStmt{ holder_name: "x".to_string(), value: l.clone(), branch: vec![], span: span() })
            ]), span: span()});

            for _ in 1..=100 {
                stmt = Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ Stmt::Expr(l.clone()) ],
                        elif_branches: vec![],
                        else_branch: Some(vec![ stmt ]),
                        span: span() });

                let result = empty_branch_analysis_hazmat(&vec![stmt.clone()]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("For loop branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_nested_for_stmt_in_if_stmt_elif_branch_errors() {
        let literals = get_all_literals();
        for l in literals {
            let mut stmt = Stmt::If(IfStmt{ condition: l.clone(), if_branch: vec![ Stmt::Expr(l.clone())], elif_branches: vec![], else_branch: Some(vec![
                Stmt::For(ForStmt{ holder_name: "x".to_string(), value: l.clone(), branch: vec![], span: span() })
            ]), span: span()});

            for _ in 1..=100 {
                stmt = Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ Stmt::Expr(l.clone()) ],
                        elif_branches: vec![(l.clone(), vec![stmt])],
                        else_branch: None,
                        span: span() });

                let result = empty_branch_analysis_hazmat(&vec![stmt.clone()]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("For loop branch has no statements"));
            }
        }
    }


    // Similar tests as above, except this time it's for If statements
    //
    #[test]
    fn empty_if_stmt_main_branch_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
                let stmt = Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![],
                        elif_branches: vec![],
                        else_branch: None,
                        span: span()
                    });

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("If statement `main` branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_if_stmt_else_branch_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
                let stmt = Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ Stmt::Expr(l.clone()) ], // dummy statement
                        elif_branches: vec![],
                        else_branch: Some(vec![]),
                        span: span()
                    });

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("If statement `else` branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_if_stmt_elif_branch_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
                let stmt = Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ Stmt::Expr(l.clone()) ], // dummy statement
                        elif_branches: vec![(l.clone(), vec![]); i],
                        else_branch: None,
                        span: span()
                    });

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("If statement `elif` branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_if_stmt_main_and_else_branch_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
                let stmt = Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![],
                        elif_branches: vec![],
                        else_branch: Some(vec![]),
                        span: span()
                    });

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("If statement `main` branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_if_stmt_main_and_elif_branch_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
                let stmt = Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![],
                        elif_branches: vec![(l.clone(), vec![]); i],
                        else_branch: Some(vec![]),
                        span: span()
                    });

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("If statement `main` branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_if_stmt_elif_and_else_branch_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
                let stmt = Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ Stmt::Expr(l.clone()) ], // dummy statement
                        elif_branches: vec![(l.clone(), vec![])],
                        else_branch: Some(vec![]),
                        span: span()
                    });

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("If statement `elif` branch has no statements"));
            }
        }
    }


    #[test]
    fn empty_nested_if_stmt_main_branch_errors() {
        let literals = get_all_literals();
        for l in literals {
            let mut stmt = Stmt::If(IfStmt{ condition: l.clone(), if_branch: vec![], elif_branches: vec![], else_branch: None, span: span()});

            for _ in 1..=100 {
                stmt = Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ stmt ],
                        elif_branches: vec![],
                        else_branch: None,
                        span: span()
                    });

                let result = empty_branch_analysis_hazmat(&vec![stmt.clone()]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("If statement `main` branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_nested_if_stmt_else_branch_errors() {
        let literals = get_all_literals();
        for l in literals {
            let mut stmt = Stmt::If(IfStmt{ condition: l.clone(), if_branch: vec![ Stmt::Expr(l.clone())], elif_branches: vec![], else_branch: Some(vec![]), span: span()});

            for _ in 1..=100 {
                stmt = Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ Stmt::Expr(l.clone()) ],
                        elif_branches: vec![],
                        else_branch: Some(vec![ stmt ]),
                        span: span()
                    });

                let result = empty_branch_analysis_hazmat(&vec![stmt.clone()]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("If statement `else` branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_nested_if_stmt_elif_branch_errors() {
        let literals = get_all_literals();
        for l in literals {
            let mut stmt = Stmt::If(IfStmt{ condition: l.clone(), if_branch: vec![ Stmt::Expr(l.clone())], elif_branches: vec![(l.clone(), vec![])], else_branch: None, span: span()});

            for _ in 1..=100 {
                stmt = Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ Stmt::Expr(l.clone()) ],
                        elif_branches: vec![(l.clone(), vec![stmt])],
                        else_branch: None,
                        span: span()
                    });

                let result = empty_branch_analysis_hazmat(&vec![stmt.clone()]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("If statement `elif` branch has no statements"));
            }
        }
    }

    //
    // Same as above tests, except, this time the if statement(s) are in an infinite loop statement
    // branch
    //
    #[test]
    fn empty_if_stmt_main_branch_in_infinite_stmt_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
                let stmt = Stmt::Infinite(InfiniteStmt{ branch: vec![
                    Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![],
                        elif_branches: vec![],
                        else_branch: None,
                        span: span() })], span: span()});

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("If statement `main` branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_if_stmt_else_branch_in_infinite_stmt_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
                let stmt = Stmt::Infinite(InfiniteStmt{ branch: vec![
                    Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ Stmt::Expr(l.clone()) ], // dummy statement
                        elif_branches: vec![],
                        else_branch: Some(vec![]),
                        span: span() })], span: span()});

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("If statement `else` branch has no statements"));
            }
        }
    }


    #[test]
    fn empty_if_stmt_elif_branch_in_infinite_stmt_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
                let stmt = Stmt::Infinite(InfiniteStmt{ branch: vec![
                    Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ Stmt::Expr(l.clone()) ], // dummy statement
                        elif_branches: vec![(l.clone(), vec![]); i],
                        else_branch: None,
                        span: span() })], span: span()});

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("If statement `elif` branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_if_stmt_main_and_else_branch_in_infinite_stmt_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
            let stmt = Stmt::Infinite(InfiniteStmt{ branch: vec![
                    Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![],
                        elif_branches: vec![],
                        else_branch: Some(vec![]),
                        span: span() })], span: span()});

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("If statement `main` branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_if_stmt_main_and_elif_branch_in_infinite_stmt_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
                let stmt = Stmt::Infinite(InfiniteStmt{ branch: vec![
                    Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![],
                        elif_branches: vec![(l.clone(), vec![]); i],
                        else_branch: Some(vec![]),
                        span: span() })], span: span()});

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("If statement `main` branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_if_stmt_elif_and_else_branch_in_infinite_stmt_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
                let stmt = Stmt::Infinite(InfiniteStmt{ branch: vec![
                    Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ Stmt::Expr(l.clone()) ], // dummy statement
                        elif_branches: vec![(l.clone(), vec![])],
                        else_branch: Some(vec![]),
                        span: span() })], span: span()});

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("If statement `elif` branch has no statements"));
            }
        }
    }


    #[test]
    fn empty_nested_if_stmt_main_branch_in_infinite_stmt_errors() {
        let literals = get_all_literals();
        for l in literals {
            let mut stmt = Stmt::Infinite(InfiniteStmt{ branch: vec![
                Stmt::If(IfStmt{ condition: l.clone(), if_branch: vec![], elif_branches: vec![], else_branch: None, span: span()})
            ], span: span()});

            for _ in 1..=100 {
                stmt = Stmt::Infinite(InfiniteStmt{ branch: vec![
                    Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ stmt ],
                        elif_branches: vec![],
                        else_branch: None,
                    span: span() })], span: span()});

                let result = empty_branch_analysis_hazmat(&vec![stmt.clone()]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("If statement `main` branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_nested_if_stmt_else_branch_in_infinite_stmt_errors() {
        let literals = get_all_literals();
        for l in literals {
            let mut stmt = Stmt::Infinite(InfiniteStmt{ branch: vec![
                Stmt::If(IfStmt{ condition: l.clone(), if_branch: vec![ Stmt::Expr(l.clone())], elif_branches: vec![], else_branch: Some(vec![]), span: span()})
            ], span: span()});

            for _ in 1..=100 {
                stmt = Stmt::Infinite(InfiniteStmt{ branch: vec![
                    Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ Stmt::Expr(l.clone()) ],
                        elif_branches: vec![],
                        else_branch: Some(vec![ stmt ]),
                        span: span() })], span: span()});

                let result = empty_branch_analysis_hazmat(&vec![stmt.clone()]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("If statement `else` branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_nested_if_stmt_elif_branch_in_infinite_stmt_errors() {
        let literals = get_all_literals();
        for l in literals {
            let mut stmt = Stmt::Infinite(InfiniteStmt{ branch: vec![
                Stmt::If(IfStmt{ condition: l.clone(), if_branch: vec![ Stmt::Expr(l.clone())], elif_branches: vec![(l.clone(), vec![])], else_branch: None, span: span()})
            ], span: span()});

            for _ in 1..=100 {
                stmt = Stmt::Infinite(InfiniteStmt{ branch: vec![
                    Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ Stmt::Expr(l.clone()) ],
                        elif_branches: vec![(l.clone(), vec![stmt])],
                        else_branch: None,
                        span: span() })], span: span()});

                let result = empty_branch_analysis_hazmat(&vec![stmt.clone()]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("If statement `elif` branch has no statements"));
            }
        }
    }


    //
    // Same as above tests, except, this time the if statement(s) are in a while loop statement
    // branch
    //
    #[test]
    fn empty_if_stmt_main_branch_in_while_stmt_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
                let stmt = Stmt::While(WhileStmt{ condition: l.clone(), branch: vec![
                    Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![],
                        elif_branches: vec![],
                        else_branch: None,
                        span: span() })], span: span()});

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("If statement `main` branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_if_stmt_else_branch_in_while_stmt_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
                let stmt = Stmt::While(WhileStmt{ condition: l.clone(), branch: vec![
                    Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ Stmt::Expr(l.clone()) ], // dummy statement
                        elif_branches: vec![],
                        else_branch: Some(vec![]),
                        span: span() })], span: span()});

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("If statement `else` branch has no statements"));
            }
        }
    }


    #[test]
    fn empty_if_stmt_elif_branch_in_while_stmt_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
                let stmt = Stmt::While(WhileStmt{ condition: l.clone(), branch: vec![
                    Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ Stmt::Expr(l.clone()) ], // dummy statement
                        elif_branches: vec![(l.clone(), vec![]); i],
                        else_branch: None,
                        span: span() })], span: span()});

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("If statement `elif` branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_if_stmt_main_and_else_branch_in_while_stmt_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
            let stmt = Stmt::While(WhileStmt{ condition: l.clone(), branch: vec![
                    Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![],
                        elif_branches: vec![],
                        else_branch: Some(vec![]),
                        span: span() })], span: span()});

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("If statement `main` branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_if_stmt_main_and_elif_branch_in_while_stmt_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
                let stmt = Stmt::While(WhileStmt{ condition: l.clone(), branch: vec![
                    Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![],
                        elif_branches: vec![(l.clone(), vec![]); i],
                        else_branch: Some(vec![]),
                        span: span() })], span: span()});

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("If statement `main` branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_if_stmt_elif_and_else_branch_in_while_stmt_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
                let stmt = Stmt::While(WhileStmt{ condition: l.clone(), branch: vec![
                    Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ Stmt::Expr(l.clone()) ], // dummy statement
                        elif_branches: vec![(l.clone(), vec![])],
                        else_branch: Some(vec![]),
                        span: span() })], span: span()});

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("If statement `elif` branch has no statements"));
            }
        }
    }


    #[test]
    fn empty_nested_if_stmt_main_branch_in_while_stmt_errors() {
        let literals = get_all_literals();
        for l in literals {
            let mut stmt = Stmt::While(WhileStmt{ condition: l.clone(), branch: vec![
                Stmt::If(IfStmt{ condition: l.clone(), if_branch: vec![], elif_branches: vec![], else_branch: None, span: span()})
            ], span: span()});

            for _ in 1..=100 {
                stmt = Stmt::While(WhileStmt{ condition: l.clone(), branch: vec![
                    Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ stmt ],
                        elif_branches: vec![],
                        else_branch: None,
                    span: span() })], span: span()});

                let result = empty_branch_analysis_hazmat(&vec![stmt.clone()]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("If statement `main` branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_nested_if_stmt_else_branch_in_while_stmt_errors() {
        let literals = get_all_literals();
        for l in literals {
            let mut stmt = Stmt::While(WhileStmt{ condition: l.clone(), branch: vec![
                Stmt::If(IfStmt{ condition: l.clone(), if_branch: vec![ Stmt::Expr(l.clone())], elif_branches: vec![], else_branch: Some(vec![]), span: span()})
            ], span: span()});

            for _ in 1..=100 {
                stmt = Stmt::While(WhileStmt{ condition: l.clone(), branch: vec![
                    Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ Stmt::Expr(l.clone()) ],
                        elif_branches: vec![],
                        else_branch: Some(vec![ stmt ]),
                        span: span() })], span: span()});

                let result = empty_branch_analysis_hazmat(&vec![stmt.clone()]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("If statement `else` branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_nested_if_stmt_elif_branch_in_while_stmt_errors() {
        let literals = get_all_literals();
        for l in literals {
            let mut stmt = Stmt::While(WhileStmt{ condition: l.clone(), branch: vec![
                Stmt::If(IfStmt{ condition: l.clone(), if_branch: vec![ Stmt::Expr(l.clone())], elif_branches: vec![(l.clone(), vec![])], else_branch: None, span: span()})
            ], span: span()});

            for _ in 1..=100 {
                stmt = Stmt::While(WhileStmt{ condition: l.clone(), branch: vec![
                    Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ Stmt::Expr(l.clone()) ],
                        elif_branches: vec![(l.clone(), vec![stmt])],
                        else_branch: None,
                        span: span() })], span: span()});

                let result = empty_branch_analysis_hazmat(&vec![stmt.clone()]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("If statement `elif` branch has no statements"));
            }
        }
    }


    //
    // Same as above tests, except, this time the if statement(s) are in a for loop statement
    // branch
    //
    #[test]
    fn empty_if_stmt_main_branch_in_for_stmt_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
                let stmt = Stmt::For(ForStmt{ holder_name: "x".to_string(), value: l.clone(), branch: vec![
                    Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![],
                        elif_branches: vec![],
                        else_branch: None,
                        span: span() })], span: span()});

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("If statement `main` branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_if_stmt_else_branch_in_for_stmt_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
                let stmt = Stmt::For(ForStmt{ holder_name: "x".to_string(), value: l.clone(), branch: vec![
                    Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ Stmt::Expr(l.clone()) ], // dummy statement
                        elif_branches: vec![],
                        else_branch: Some(vec![]),
                        span: span() })], span: span()});

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("If statement `else` branch has no statements"));
            }
        }
    }


    #[test]
    fn empty_if_stmt_elif_branch_in_for_stmt_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
                let stmt = Stmt::For(ForStmt{ holder_name: "x".to_string(), value: l.clone(), branch: vec![
                    Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ Stmt::Expr(l.clone()) ], // dummy statement
                        elif_branches: vec![(l.clone(), vec![]); i],
                        else_branch: None,
                        span: span() })], span: span()});

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("If statement `elif` branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_if_stmt_main_and_else_branch_in_for_stmt_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
            let stmt = Stmt::For(ForStmt{ holder_name: "x".to_string(), value: l.clone(), branch: vec![
                    Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![],
                        elif_branches: vec![],
                        else_branch: Some(vec![]),
                        span: span() })], span: span()});

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("If statement `main` branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_if_stmt_main_and_elif_branch_in_for_stmt_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
                let stmt = Stmt::For(ForStmt{ holder_name: "x".to_string(), value: l.clone(), branch: vec![
                    Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![],
                        elif_branches: vec![(l.clone(), vec![]); i],
                        else_branch: Some(vec![]),
                        span: span() })], span: span()});

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("If statement `main` branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_if_stmt_elif_and_else_branch_in_for_stmt_errors() {
        let literals = get_all_literals();
        for l in literals {
            for i in 1..100 {
                let stmt = Stmt::For(ForStmt{ holder_name: "x".to_string(), value: l.clone(), branch: vec![
                    Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ Stmt::Expr(l.clone()) ], // dummy statement
                        elif_branches: vec![(l.clone(), vec![])],
                        else_branch: Some(vec![]),
                        span: span() })], span: span()});

                let result = empty_branch_analysis_hazmat(&vec![stmt; i]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("If statement `elif` branch has no statements"));
            }
        }
    }


    #[test]
    fn empty_nested_if_stmt_main_branch_in_for_stmt_errors() {
        let literals = get_all_literals();
        for l in literals {
            let mut stmt = Stmt::For(ForStmt{ holder_name: "x".to_string(), value: l.clone(), branch: vec![
                Stmt::If(IfStmt{ condition: l.clone(), if_branch: vec![], elif_branches: vec![], else_branch: None, span: span()})
            ], span: span()});

            for _ in 1..=100 {
                stmt = Stmt::For(ForStmt{ holder_name: "x".to_string(), value: l.clone(), branch: vec![
                    Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ stmt ],
                        elif_branches: vec![],
                        else_branch: None,
                    span: span() })], span: span()});

                let result = empty_branch_analysis_hazmat(&vec![stmt.clone()]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("If statement `main` branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_nested_if_stmt_else_branch_in_for_stmt_errors() {
        let literals = get_all_literals();
        for l in literals {
            let mut stmt = Stmt::For(ForStmt{ holder_name: "x".to_string(), value: l.clone(), branch: vec![
                Stmt::If(IfStmt{ condition: l.clone(), if_branch: vec![ Stmt::Expr(l.clone())], elif_branches: vec![], else_branch: Some(vec![]), span: span()})
            ], span: span()});

            for _ in 1..=100 {
                stmt = Stmt::For(ForStmt{ holder_name: "x".to_string(), value: l.clone(), branch: vec![
                    Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ Stmt::Expr(l.clone()) ],
                        elif_branches: vec![],
                        else_branch: Some(vec![ stmt ]),
                        span: span() })], span: span()});

                let result = empty_branch_analysis_hazmat(&vec![stmt.clone()]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("If statement `else` branch has no statements"));
            }
        }
    }

    #[test]
    fn empty_nested_if_stmt_elif_branch_in_for_stmt_errors() {
        let literals = get_all_literals();
        for l in literals {
            let mut stmt = Stmt::For(ForStmt{ holder_name: "x".to_string(), value: l.clone(), branch: vec![
                Stmt::If(IfStmt{ condition: l.clone(), if_branch: vec![ Stmt::Expr(l.clone())], elif_branches: vec![(l.clone(), vec![])], else_branch: None, span: span()})
            ], span: span()});

            for _ in 1..=100 {
                stmt = Stmt::For(ForStmt{ holder_name: "x".to_string(), value: l.clone(), branch: vec![
                    Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![ Stmt::Expr(l.clone()) ],
                        elif_branches: vec![(l.clone(), vec![stmt])],
                        else_branch: None,
                        span: span() })], span: span()});

                let result = empty_branch_analysis_hazmat(&vec![stmt.clone()]);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("If statement `elif` branch has no statements"));
            }
        }
    }





}
