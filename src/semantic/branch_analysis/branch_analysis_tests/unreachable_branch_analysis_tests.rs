use super::*;

#[cfg(test)]
mod unreachable_branch_analysis_tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Compiler bug")]
    fn empty_func_branch_panics() {
        let stmts: Vec<Stmt> = vec![];
        let _ = dead_code_analysis(&stmts, false);
    }


    #[test]
    fn empty_for_statement_branch_dead() {
        let literals_with_var = get_all_literals_with_var_no_arr();

        for lv in literals_with_var {
            let stmts: Vec<Stmt> = vec![
                Stmt::For(ForStmt{
                    holder_name: "x".to_string(),
                    value: lv,
                    branch: vec![],
                    span: span(),
                })
            ];

            let result = dead_code_analysis(&stmts, false);
            // Block has no dead code (because while statement may or may not execute).
            // But inside the while statement its self, its empty, so it its self is dead.
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().starts_with("Semantic error: For loop branch has no statements. Empty branches are not allowed"));
        }
    }

    #[test]
    fn for_statement_branch_multiple_return_errors() {
        let literals_with_var = get_all_literals_with_var_no_arr();

        for lv in literals_with_var {
            let stmt = Stmt::Expr(lv.clone());
            for i in 0..=1000 {
                let mut dummy_branch = vec![stmt.clone(); i + 1];
            
                // Insert return statement at `i`
                let rstmt = make_return_stmt(vec![lv.clone()]);
                dummy_branch.insert(i, rstmt);

                let stmts: Vec<Stmt> = vec![
                    Stmt::For(ForStmt{
                        holder_name: "x".to_string(),
                        value: lv.clone(),
                        branch: dummy_branch,
                        span: span(),
                    })
                ];

                let result = dead_code_analysis(&stmts, false);

                // Block has no dead code (because for statement may or may not execute).
                // But inside the for statement its self, there are statements after the return
                // statement, so those are dead.
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Dead code detected starting from line"));

            }       
        }
    }

    #[test]
    fn empty_while_statement_branch_dead() {
        let literals_with_var = get_all_literals_with_var_no_arr();

        for lv in literals_with_var {
            let stmts: Vec<Stmt> = vec![
                Stmt::While(WhileStmt{
                    condition: lv,
                    branch: vec![],
                    span: span(),
                })
            ];

            let result = dead_code_analysis(&stmts, false);
            // Block has no dead code (because while statement may or may not execute).
            // But inside the while statement its self, its empty, so it its self is dead.
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().starts_with("Semantic error: While loop branch has no statements. Empty branches are not allowed"));
     
        }
    }


    #[test]
    fn while_statement_branch_multiple_return_errors() {
        let literals_with_var = get_all_literals_with_var_no_arr();

        for lv in literals_with_var {
            let stmt = Stmt::Expr(lv.clone());
            for i in 0..=1000 {
                let mut dummy_branch = vec![stmt.clone(); i + 1];
            
                // Insert return statement at `i`
                let rstmt = make_return_stmt(vec![lv.clone()]);
                dummy_branch.insert(i, rstmt);

                let stmts: Vec<Stmt> = vec![
                    Stmt::While(WhileStmt{
                        condition: lv.clone(),
                        branch: dummy_branch,
                        span: span(),
                    })
                ];

                let result = dead_code_analysis(&stmts, false);

                // Block has no dead code (because while statement may or may not execute).
                // But inside the while statement its self, there are statements after the return
                // statement, so those are dead.
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Dead code detected starting from line"));

            }       
        }
    }

    #[test]
    fn empty_if_statement_branch_errors() {
        let literals = get_all_literals_with_var_no_arr();

        for l in literals {
            let stmts: Vec<Stmt> = vec![
                Stmt::If(IfStmt{
                    condition: l,
                    if_branch: vec![],
                    elif_branches: vec![],
                    else_branch: None,
                    span: span(),
                })
            ];

            let result = dead_code_analysis(&stmts, false);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("If statement main branch has no statements."));
        }
    }

    #[test]
    fn empty_if_statement_else_branch_errors() {
        let literals = get_all_literals_with_var_no_arr();

        for l in literals {
            let stmts: Vec<Stmt> = vec![
                Stmt::If(IfStmt{
                    condition: l.clone(),
                    if_branch: vec![ Stmt::Expr(l) ],
                    elif_branches: vec![],
                    else_branch: Some(vec![]),
                    span: span(),
                })
            ];

            let result = dead_code_analysis(&stmts, false);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("If statement `else` branch has no statements"));
        }
    }

    #[test]
    fn empty_if_statement_elif_branch_errors() {
        let literals = get_all_literals_with_var_no_arr();

        for l in literals {
            let stmts: Vec<Stmt> = vec![
                Stmt::If(IfStmt{
                    condition: l.clone(),
                    if_branch: vec![ Stmt::Expr(l.clone()) ],
                    elif_branches: vec![
                        (l, vec![])
                    ],
                    else_branch: None,
                    span: span(),
                })
            ];

            let result = dead_code_analysis(&stmts, false);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("If statement `elif` branch has no statements"));
        }
    }


    #[test]
    fn if_statement_main_branch_multiple_return_errors() {
        let literals_with_var = get_all_literals_with_var_no_arr();

        for lv in literals_with_var {
            let stmt = Stmt::Expr(lv.clone());
            for i in 0..=1000 {
                let mut dummy_branch = vec![stmt.clone(); i + 1];
            
                // Insert return statement at `i`
                let rstmt = make_return_stmt(vec![lv.clone()]);
                dummy_branch.insert(i, rstmt);

                let stmts: Vec<Stmt> = vec![
                    Stmt::If(IfStmt{
                        condition: lv.clone(),
                        if_branch: dummy_branch,
                        elif_branches: vec![],
                        else_branch: None,
                        span: span(),
                    })
                ];

                let result = dead_code_analysis(&stmts, false);

                // Block has no dead code (because if statement may or may not execute).
                // But inside the if statement main branch its self, there are statements after the return
                // statement, so those are dead.
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Dead code detected starting from line"));

            }       
        }
    }


    #[test]
    fn if_statement_else_branch_multiple_return_errors() {
        let literals_with_var = get_all_literals_with_var_no_arr();

        for lv in literals_with_var {
            let stmt = Stmt::Expr(lv.clone());
            for i in 0..=1000 {
                let mut dummy_branch = vec![stmt.clone(); i + 1];
            
                // Insert return statement at `i`
                let rstmt = make_return_stmt(vec![lv.clone()]);
                dummy_branch.insert(i, rstmt);

                let stmts: Vec<Stmt> = vec![
                    Stmt::If(IfStmt{
                        condition: lv.clone(),
                        if_branch: vec![ Stmt::Expr(lv.clone()) ],
                        elif_branches: vec![],
                        else_branch: Some(dummy_branch),
                        span: span(),
                    })
                ];

                let result = dead_code_analysis(&stmts, false);

                // Block has no dead code (because if statement else branch may or may not execute).
                // But inside the if statement else branch its self, there are statements after the return
                // statement, so those are dead.
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Dead code detected starting from line"));

            }       
        }
    }


    #[test]
    fn if_statement_elif_branch_multiple_return_errors() {
        let literals_with_var = get_all_literals_with_var_no_arr();

        for lv in literals_with_var {
            let stmt = Stmt::Expr(lv.clone());
            for i in 0..=1000 {
                let mut dummy_branch = vec![stmt.clone(); i + 1];
            
                // Insert return statement at `i`
                let rstmt = make_return_stmt(vec![lv.clone()]);
                dummy_branch.insert(i, rstmt);

                let stmts: Vec<Stmt> = vec![
                    Stmt::If(IfStmt{
                        condition: lv.clone(),
                        if_branch: vec![ Stmt::Expr(lv.clone()) ],
                        elif_branches: vec![ (lv.clone(), dummy_branch) ],
                        else_branch: None,
                        span: span(),
                    })
                ];

                let result = dead_code_analysis(&stmts, false);

                // Block has no dead code (because if statement elif branch may or may not execute).
                // But inside the if statement elif branch its self, there are statements after the return
                // statement, so those are dead.
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Dead code detected starting from line"));
            }
        }
    }


    #[test]
    fn if_statement_elif_branch_returns() {
        let literals = get_all_literals_with_var_no_arr();

        for l in literals {
            let stmts: Vec<Stmt> = vec![
                Stmt::If(IfStmt{
                    condition: l.clone(),
                    if_branch: vec![ Stmt::Expr(l.clone()) ],
                    elif_branches: vec![
                        (l.clone(), vec![ make_return_stmt(vec![l.clone()]) ])
                    ],
                    else_branch: Some(vec![ Stmt::Expr(l.clone()) ]),
                    span: span(),
                })
            ];

            let result = dead_code_analysis(&stmts, false);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn if_statement_main_elif_branch_returns() {
        let literals = get_all_literals_with_var_no_arr();

        for l in literals {
            let stmts: Vec<Stmt> = vec![
                Stmt::If(IfStmt{
                    condition: l.clone(),
                    if_branch: vec![ make_return_stmt(vec![l.clone()]) ],
                    elif_branches: vec![
                        (l.clone(), vec![ make_return_stmt(vec![l.clone()]) ])
                    ],
                    else_branch: Some(vec![ Stmt::Expr(l.clone()) ]),
                    span: span(),
                })
            ];

            let result = dead_code_analysis(&stmts, false);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn if_statement_main_else_branch_returns() {
        let literals = get_all_literals_with_var_no_arr();

        for l in literals {
            let stmts: Vec<Stmt> = vec![
                Stmt::If(IfStmt{
                    condition: l.clone(),
                    if_branch: vec![ make_return_stmt(vec![l.clone()]) ],
                    elif_branches: vec![
                        (l.clone(), vec![ Stmt::Expr(l.clone()) ]),
                    ],
                    else_branch: Some(vec![ make_return_stmt(vec![l.clone()]) ]),
                    span: span(),
                })
            ];

            let result = dead_code_analysis(&stmts, false);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn if_statement_else_elif_branches_returns() {
        let literals = get_all_literals_with_var_no_arr();

        for l in literals {
            let stmts: Vec<Stmt> = vec![
                Stmt::If(IfStmt{
                    condition: l.clone(),
                    if_branch: vec![ Stmt::Expr(l.clone()) ],
                    elif_branches: vec![
                        (l.clone(), vec![ make_return_stmt(vec![l.clone()]) ])
                    ],
                    else_branch: Some(vec![ make_return_stmt(vec![l.clone()]) ]),
                    span: span(),
                })
            ];

            let result = dead_code_analysis(&stmts, false);
            assert!(result.is_ok());
        }
    }


    #[test]
    fn if_statement_main_else_elif_branches_returns() {
        let literals = get_all_literals_with_var_no_arr();

        for l in literals {
            let stmts: Vec<Stmt> = vec![
                Stmt::If(IfStmt{
                    condition: l.clone(),
                    if_branch: vec![ make_return_stmt(vec![l.clone()])],
                    elif_branches: vec![
                        (l.clone(), vec![ make_return_stmt(vec![l.clone()]) ])
                    ],
                    else_branch: Some(vec![ make_return_stmt(vec![l.clone()]) ]),
                    span: span(),
                })
            ];

            let result = dead_code_analysis(&stmts, false);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn if_statement_branch_has_empty_infinite_stmt_branch_errors() {
        let literals = get_all_literals_with_var_no_arr();

        for l in literals {
            let stmts: Vec<Stmt> = vec![
                Stmt::If(IfStmt{
                    condition: l,
                    if_branch: vec![
                        Stmt::Infinite(InfiniteStmt{
                            branch: vec![],
                            span: span(),
                        })
                    ],
                    elif_branches: vec![],
                    else_branch: None,
                    span: span(),
                })
            ];

            let result = dead_code_analysis(&stmts, false);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Infinite loop branch has no statements."));
        }
    }

    #[test]
    fn if_statement_branch_has_empty_while_stmt_branch_errors() {
        let literals = get_all_literals_with_var_no_arr();

        for l in literals {
            let stmts: Vec<Stmt> = vec![
                Stmt::If(IfStmt{
                    condition: l.clone(),
                    if_branch: vec![
                        Stmt::While(WhileStmt{
                            condition: l,
                            branch: vec![],
                            span: span(),
                        })
                    ],
                    elif_branches: vec![],
                    else_branch: None,
                    span: span(),
                })
            ];

            let result = dead_code_analysis(&stmts, false);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("While loop branch has no statements."));
        }
    }

    #[test]
    fn if_statement_branch_has_empty_for_stmt_branch_errors() {
        let literals = get_all_literals_with_var_no_arr();

        for l in literals {
            let stmts: Vec<Stmt> = vec![
                Stmt::If(IfStmt{
                    condition: l.clone(),
                    if_branch: vec![
                        Stmt::For(ForStmt{
                            holder_name: "x".to_string(),
                            value: l,
                            branch: vec![],
                            span: span(),
                        })
                    ],
                    elif_branches: vec![],
                    else_branch: None,
                    span: span(),
                })
            ];

            let result = dead_code_analysis(&stmts, false);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("For loop branch has no statements."));
        }
    }

    #[test]
    fn if_statement_elif_branch_has_empty_infinite_stmt_branch_errors() {
        let literals = get_all_literals_with_var_no_arr();

        for l in literals {
            let stmts: Vec<Stmt> = vec![
                Stmt::If(IfStmt{
                    condition: l.clone(),
                    if_branch: vec![ Stmt::Expr(l.clone()) ],
                    elif_branches: vec![
                        (l.clone(), vec![
                        Stmt::Infinite(InfiniteStmt{
                            branch: vec![],
                            span: span(),
                        })
                    ])],
                    else_branch: None,
                    span: span(),
                })
            ];

            let result = dead_code_analysis(&stmts, false);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Infinite loop branch has no statements."));
        }
    }

    #[test]
    fn if_statement_elif_branch_has_empty_while_stmt_branch_errors() {
        let literals = get_all_literals_with_var_no_arr();

        for l in literals {
            let stmts: Vec<Stmt> = vec![
                Stmt::If(IfStmt{
                    condition: l.clone(),
                    if_branch: vec![ Stmt::Expr(l.clone()) ],
                    elif_branches: vec![
                        (l.clone(), vec![
                        Stmt::While(WhileStmt{
                            condition: l,
                            branch: vec![],
                            span: span(),
                        })
                    ])],
                    else_branch: None,
                    span: span(),
                })
            ];

            let result = dead_code_analysis(&stmts, false);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("While loop branch has no statements."));
        }
    }

    #[test]
    fn if_statement_elif_branch_has_empty_for_stmt_branch_errors() {
        let literals = get_all_literals_with_var_no_arr();

        for l in literals {
            let stmts: Vec<Stmt> = vec![
                Stmt::If(IfStmt{
                    condition: l.clone(),
                    if_branch: vec![ Stmt::Expr(l.clone()) ],
                    elif_branches: vec![
                        (l.clone(), vec![ 
                            Stmt::For(ForStmt{
                                holder_name: "x".to_string(),
                                value: l,
                                branch: vec![],
                                span: span(),
                            })
                        ])],
                    else_branch: None,
                    span: span(),
                })
            ];

            let result = dead_code_analysis(&stmts, false);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("For loop branch has no statements."));
        }
    }






    #[test]
    fn empty_infinite_statement_branch() {
        let stmts: Vec<Stmt> = vec![
            Stmt::Infinite(InfiniteStmt{
                branch: vec![],
                span: span(),
            })
        ];

        let result = dead_code_analysis(&stmts, false);
        // Block has dead code (because of empty branch).
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Infinite loop branch has no statements."));
    }



    #[test]
    fn infinite_statement_branch_not_dead() {
        let literals_with_var = get_all_literals_with_var_no_arr();

        for lv in literals_with_var {
            let stmt = Stmt::Expr(lv.clone());
            for i in 0..=1000 {
                let dummy_branch = vec![stmt.clone(); i + 1];

                let stmts: Vec<Stmt> = vec![
                    Stmt::Infinite(InfiniteStmt{
                        branch: dummy_branch,
                        span: span(),
                    })
                ];

                let result = dead_code_analysis(&stmts, false);
                // Block has no dead code.
                assert!(result.is_ok());
            }
        }
    }

    #[test]
    fn infinite_statement_branch_return_one_time() {
        let literals_with_var = get_all_literals_with_var_no_arr();

        for lv in literals_with_var {
            let stmts: Vec<Stmt> = vec![
                Stmt::Infinite(InfiniteStmt{
                    branch: vec![ make_return_stmt(vec![lv.clone()]) ],
                    span: span(),
                })
            ];

            let result = dead_code_analysis(&stmts, false);
            // Block has no dead code.
            assert!(result.is_ok());
        }
    }



    #[test]
    fn infinite_statement_branch_return_multiple_times_dead() {
        let literals_with_var = get_all_literals_with_var_no_arr();

        for lv in literals_with_var {
            let stmt = make_return_stmt(vec![lv.clone()]);
            for i in 1..=1000 {
                let dummy_branch = vec![stmt.clone(); i + 1];

                let stmts: Vec<Stmt> = vec![
                    Stmt::Infinite(InfiniteStmt{
                        branch: dummy_branch,
                        span: span(),
                    })
                ];

                let result = dead_code_analysis(&stmts, false);
                // Block has dead code because it returns more than once.
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().starts_with("Semantic error: Dead code detected starting from line"));
            }
        }
    }


    #[test]
    fn infinite_statement_branch_stmts_after_return_dead() {
        let literals_with_var = get_all_literals_with_var_no_arr();

        for lv in literals_with_var {
            let stmt = Stmt::Expr(lv.clone());
            for i in 0..=1000 {
                let mut dummy_branch = vec![stmt.clone(); i + 1];
            
                // Insert return statement at `i`
                let rstmt = make_return_stmt(vec![lv.clone()]);
                dummy_branch.insert(i, rstmt);

                let stmts: Vec<Stmt> = vec![
                    Stmt::Infinite(InfiniteStmt{
                        branch: dummy_branch,
                        span: span(),
                    })
                ];

                let result = dead_code_analysis(&stmts, false);
                // Block has dead code because it contains statements after the certain return.
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().starts_with("Semantic error: Dead code detected starting from line"));
            }
        }
    }




    #[test]
    fn infinite_statement_branch_break_multiple_times_dead() {
        for i in 1..=1000 {
            let dummy_branch = vec![make_break_stmt(); i + 1];

            let stmts: Vec<Stmt> = vec![
                Stmt::Infinite(InfiniteStmt{
                    branch: dummy_branch,
                    span: span(),
                })
            ];

            let result = dead_code_analysis(&stmts, false);
            // Block has dead code because it returns more than once.
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().starts_with("Semantic error: Dead code detected starting from line"));
        }
    }


    #[test]
    fn infinite_statement_branch_stmts_after_break_dead() {
        let literals_with_var = get_all_literals_with_var_no_arr();

        for lv in literals_with_var {
            let stmt = Stmt::Expr(lv.clone());
            for i in 0..=1000 {
                let mut dummy_branch = vec![stmt.clone(); i + 1];
            
                // Insert break statement at `i`
                let bstmt = make_break_stmt();
                dummy_branch.insert(i, bstmt);

                let stmts: Vec<Stmt> = vec![
                    Stmt::Infinite(InfiniteStmt{
                        branch: dummy_branch,
                        span: span(),
                    })
                ];

                let result = dead_code_analysis(&stmts, false);
                // Block has dead code because it contains statements after the certain return.
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Dead code detected starting from line"));
            }
        }
    }
}

