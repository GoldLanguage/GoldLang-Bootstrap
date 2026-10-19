use super::*;

#[cfg(test)]
mod return_branch_analysis_hazmat_wrapper_tests {
    use super::*;

    #[test]
    #[should_panic]
    fn func_is_empty_panics() {
        let dummy_func = Function { 
            name: "foo".to_string(), params: vec![], return_type: None, body: vec![], span: span()
        };

        let _ = return_branch_analysis_hazmat_wrapper(&dummy_func);
    }

    #[test]
    fn func_never_returns() {
        let literals = get_all_literals_with_var_and_var_arr();
        
        for l in literals {
            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                let dummy_func = returning_func(&"foo", vec![], vec![t.clone()], vec![Stmt::Expr(l.clone())]);

                let result = return_branch_analysis_hazmat_wrapper(&dummy_func);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Expected function `foo` to return, but we found no return statements"));
            }
        }
    }

    #[test]
    fn func_returns() {
        let literals = get_all_literals_with_var_and_var_arr();

        for l in literals {
            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                let dummy_func = returning_func(&"foo", vec![], vec![t.clone()], vec![
                    return_stmt(vec![l.clone()])
                ]);


                let result = return_branch_analysis_hazmat_wrapper(&dummy_func);
                assert!(result.is_ok());
            }
        }
    }

    // A lil c00l panic guard to prevent retarded callers (such as myself) :)
    #[should_panic]
    #[test]
    fn void_function_panics() {
        let dummy_func = void_func(&"foo", vec![], vec![]);

        let _ = return_branch_analysis_hazmat_wrapper(&dummy_func);
    }

    // Empty branches should panic because return_branch_analysis_hazmat_wrapper assumes 
    // all function branches contain at least 1 statement, which
    // is what is guaranteed by code_analysis order of function calls (i.e. empty branch analysis is done before return branch analysis).
    #[should_panic]
    #[test]
    fn empty_function_panics() {
        for t in ALL_TYPES_WITH_DYN_ARR.iter() {
            let dummy_func = returning_func(&"foo", vec![], vec![t.clone()], vec![]);

            let _ = return_branch_analysis_hazmat_wrapper(&dummy_func);
        }
    }

    #[should_panic]
    #[test]
    fn infinite_statement_empty_branch_panics() {
        for t in ALL_TYPES_WITH_DYN_ARR.iter() {
            let dummy_func = returning_func(&"foo", vec![], vec![t.clone()], vec![
                Stmt::Infinite(InfiniteStmt{
                    branch: vec![],
                    span: span(),
                })
            ]);

            let _ = return_branch_analysis_hazmat_wrapper(&dummy_func);
        }
    }
    
    // Same as above test, but this time nested.
    #[test]
    fn infinite_statement_nested_branch_empty_panics() {
        for t in ALL_TYPES_WITH_DYN_ARR.iter() {
            // NOTE: This test unlike other tests doesn't build from inside out, because return analysis
            // doesn't care too much about deeply nested infinite loops, it only cares about 2 nested
            // loops max
            // Which I think is something worth changing, but for now, this test will be fine.
            let stmts = vec![
                Stmt::Infinite(InfiniteStmt {
                    branch: vec![
                        Stmt::Infinite(InfiniteStmt {
                            branch: vec![],
                            span: span()
                        })
                    ],
                    span: span(),
                })
            ];
            
            let dummy_func = returning_func(&"foo", vec![], vec![t.clone()], stmts);

            let result = std::panic::catch_unwind(|| {
                let _ = return_branch_analysis_hazmat_wrapper(&dummy_func);
            });

            assert!(result.is_err(), "Expected panic for: {:?}", dummy_func);
        }
    }

    // If you try to break in an infinite loop that is last statement, it must error.
    //
    #[test]
    fn infinite_statement_break_errors() {
        for t in ALL_TYPES_WITH_DYN_ARR.iter() {       
            let dummy_func = returning_func(&"foo", vec![], vec![t.clone()], vec![
                Stmt::Infinite(InfiniteStmt{
                    branch: vec![
                        break_stmt()
                    ],
                    span: span(),
                })
            ]);

            let result = return_branch_analysis_hazmat_wrapper(&dummy_func);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Cannot break out of infinite loop since it is the last statement in a returning function"));
        }
    }

    // Same as above, but this is an if statement, inside infinite statement..
    #[test]
    fn if_statement_main_branch_inside_infinite_stmt_break_errors() {
        let literals = get_all_literals_with_var_and_var_arr();
        for l in literals {
            for t in ALL_TYPES_WITH_DYN_ARR.iter() {            
                let dummy_func = returning_func(&"foo", vec![], vec![t.clone()],vec![
                    Stmt::Infinite(InfiniteStmt{
                        branch: vec![
                            Stmt::If(IfStmt{
                                condition: l.clone(),
                                if_branch: vec![break_stmt()],
                                elif_branches: vec![],
                                else_branch: None,
                                span: span(),
                            }),

                        ],
                        span: span(),
                    })
                ]);

                let result = return_branch_analysis_hazmat_wrapper(&dummy_func);

                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Cannot break out of infinite loop since it is the last statement in a returning function"));
            }
        }
    }

    #[test]
    fn if_statement_else_branch_inside_infinite_stmt_break_errors() {
        let literals = get_all_literals_with_var_and_var_arr();
        for l in literals {
            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                let dummy_func = returning_func(&"foo", vec![], vec![t.clone()],vec![
                    Stmt::Infinite(InfiniteStmt{
                        branch: vec![
                            Stmt::If(IfStmt{
                                condition: l.clone(),
                                if_branch: vec![Stmt::Expr(l.clone())],
                                elif_branches: vec![],
                                else_branch: Some(vec![break_stmt()]),
                                span: span(),
                            }),
                        ],
                        span: span(),
                    })
                ]);

                let result = return_branch_analysis_hazmat_wrapper(&dummy_func);

                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Cannot break out of infinite loop since it is the last statement in a returning function"));
            }
        }
    }

    #[test]
    fn if_statement_elif_branch_inside_infinite_stmt_break_errors() {
        let literals = get_all_literals_with_var_and_var_arr();
        for l in literals {
            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                let dummy_func = returning_func(&"foo", vec![], vec![t.clone()],vec![
                    Stmt::Infinite(InfiniteStmt{
                        branch: vec![
                            Stmt::If(IfStmt{
                                condition: l.clone(),
                                if_branch: vec![Stmt::Expr(l.clone())],
                                elif_branches: vec![
                                    (l.clone(), vec![break_stmt()]),
                                ],
                                else_branch: None,
                                span: span(),
                            }),

                        ],
                        span: span(),
                    })
                ]);

                let result = return_branch_analysis_hazmat_wrapper(&dummy_func);

                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Cannot break out of infinite loop since it is the last statement in a returning function"));
            }
        }
    }

    #[test]
    fn if_statement_main_branch_inside_infinite_stmt() {
        let literals = get_all_literals_with_var_and_var_arr();
        for l in literals {
            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                let dummy_func = returning_func(&"foo", vec![], vec![t.clone()],vec![
                    Stmt::Infinite(InfiniteStmt{
                        branch: vec![
                            Stmt::If(IfStmt{
                                condition: l.clone(),
                                if_branch: vec![Stmt::Expr(l.clone())],
                                elif_branches: vec![],
                                else_branch: None,
                                span: span(),
                            }),

                        ],
                        span: span(),
                    })
                ]);

                let result = return_branch_analysis_hazmat_wrapper(&dummy_func);
                assert!(result.is_ok());
            }
        }
    }
    
    #[test]
    fn if_statement_else_branch_inside_infinite_stmt() {
        let literals = get_all_literals_with_var_and_var_arr();
        for l in literals {
            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                let dummy_func = returning_func(&"foo", vec![], vec![t.clone()],vec![
                    Stmt::Infinite(InfiniteStmt{
                        branch: vec![
                            Stmt::If(IfStmt{
                                condition: l.clone(),
                                if_branch: vec![Stmt::Expr(l.clone())],
                                elif_branches: vec![],
                                else_branch: Some(vec![ Stmt::Expr(l.clone()) ]),
                                span: span(),
                            }),

                        ],
                        span: span(),
                    })
                ]);

                let result = return_branch_analysis_hazmat_wrapper(&dummy_func);
                assert!(result.is_ok());
            }
        }
    }

    #[test]
    fn if_statement_elif_branch_inside_infinite_stmt() {
        let literals = get_all_literals_with_var_and_var_arr();
        for l in literals {
            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                let dummy_func = returning_func(&"foo", vec![], vec![t.clone()],vec![
                    Stmt::Infinite(InfiniteStmt{
                        branch: vec![
                            Stmt::If(IfStmt{
                                condition: l.clone(),
                                if_branch: vec![Stmt::Expr(l.clone())],
                                elif_branches: vec![
                                    (l.clone(), vec![ Stmt::Expr(l.clone()) ]),
                                ],
                                else_branch: None,
                                span: span(),
                            }),

                        ],
                        span: span(),
                    })
                ]);


                let result = return_branch_analysis_hazmat_wrapper(&dummy_func);
                assert!(result.is_ok());
            }
        }
    }

    #[test]
    fn infinite_statement_inside_if_stmt_break_errors() {
        let literals = get_all_literals_with_var_and_var_arr();
        for l in literals {
            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                let dummy_func = returning_func(&"foo", vec![], vec![t.clone()],vec![
                    Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![
                            Stmt::Infinite(InfiniteStmt{
                                branch: vec![
                                    break_stmt()
                                ],
                                span: span(),
                            })
                        ],
                        elif_branches: vec![],
                        else_branch: None,
                        span: span(),
                    })
                ]);

                let result = return_branch_analysis_hazmat_wrapper(&dummy_func);

                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Cannot break out of infinite loop since it is the last statement in a returning function"));
            }
        }
    }


    // Nested infinite loops inside infinite loops breaks shouldn't be counted as breaks upstream
    #[test]
    fn infinite_statement_nested_branch_break() {
        for t in ALL_TYPES_WITH_DYN_ARR.iter() {
            for i in 1..=100 {
                // Build from the inside out
                let mut stmts: Vec<Stmt> = vec![break_stmt()];
                for _ in 0..=i {
                    stmts = vec![
                        Stmt::Infinite(InfiniteStmt {
                            branch: stmts,
                            span: span(),
                        })
                    ];
                }
            
                let dummy_func = returning_func(&"foo", vec![], vec![t.clone()],stmts);

                let result = return_branch_analysis_hazmat_wrapper(&dummy_func);

                assert!(result.is_ok());
            }
        }
    }

    
    // Nested while loops inside infinite loops breaks shouldn't be counted as breaks upstream
    #[test]
    fn infinite_statement_while_statement_nested_branch_break() {
        let literals = get_all_literals_with_var_and_var_arr();

        for l in literals {
            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                for i in 1..=100 {
                    // Build from the inside out
                    let mut stmts: Vec<Stmt> = vec![break_stmt()];
                    for _ in 0..=i {
                        stmts = vec![
                            Stmt::While(WhileStmt {
                                condition: l.clone(),
                                branch: stmts,
                                span: span(),
                            })
                        ];
                    }

                    stmts = vec![
                            Stmt::Infinite(InfiniteStmt {
                                branch: stmts,
                                span: span(),
                            })
                        ];
                
                    let dummy_func = returning_func(&"foo", vec![], vec![t.clone()],stmts);

                    let result = return_branch_analysis_hazmat_wrapper(&dummy_func);

                    assert!(result.is_ok());
                }
            }
        }
    }


    // Nested for loops inside infinite loops breaks shouldn't be counted as breaks upstream
    #[test]
    fn infinite_statement_for_statement_nested_branch_break() {
        let literals = get_all_literals_with_var_and_var_arr();

        for l in literals {
            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                for i in 1..=100 {
                    // Build from the inside out
                    let mut stmts: Vec<Stmt> = vec![break_stmt()];
                    for _ in 0..=i {
                        stmts = vec![
                            Stmt::For(ForStmt{
                                holder_name: "y".to_string(),
                                value: l.clone(),
                                branch: stmts,
                                span: span(),
                            })
                        ];
                    }

                    stmts = vec![
                        Stmt::Infinite(InfiniteStmt {
                            branch: stmts,
                            span: span(),
                        })
                    ];

                
                    let dummy_func = returning_func(&"foo", vec![], vec![t.clone()],stmts);
                    
                    let result = return_branch_analysis_hazmat_wrapper(&dummy_func);

                    assert!(result.is_ok());
                }
            }
        }
    }


    // While loops may or may not execute, therefore even if they return inside their body, the
    // function its self may not return, therefore return analysis should error here
    // 
    #[test]
    fn while_statement_returns_error() {
        let literals = get_all_literals_with_var_and_var_arr();
        for l in literals {
            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                let dummy_func = returning_func(&"foo", vec![], vec![t.clone()],vec![
                    Stmt::While(WhileStmt {
                        condition: l.clone(),
                        branch: vec![
                            return_stmt(vec![l.clone()])
                        ],
                        span: span(),
                    })
                ]);
                

                let result = return_branch_analysis_hazmat_wrapper(&dummy_func);

                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains(
                        "While loop statements may or may not execute at all, therefore it cannot be the last statement in a returning function"));
            }
        }
    }


    // For loops may or may not execute, therefore even if they return inside their body, the
    // function its self may not return, therefore return analysis should error here
    // 
    #[test]
    fn for_statement_returns_error() {
        let literals = get_all_literals_with_var_and_var_arr();
        for l in literals {
            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                let dummy_func = returning_func(&"foo", vec![], vec![t.clone()],vec![
                    Stmt::For(ForStmt {
                        holder_name: "y".to_string(),
                        value: l.clone(),
                        branch: vec![
                            return_stmt(vec![l.clone()])
                        ],
                        span: span(),
                    })
                ]);

                let result = return_branch_analysis_hazmat_wrapper(&dummy_func);

                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains(
                        "For loop statement may or may not execute at all, therefore it cannot be the last statement in a returning function"));
            }
        }
    }

    // If statement without an else branch, may or may not execute, therefore even if they return inside the main branch, the
    // function its self may not always return, therefore return analysis should error here
    // 
    #[test]
    fn if_statement_returns_error() {
        let literals = get_all_literals_with_var_and_var_arr();
        for l in literals {
            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                let dummy_func = returning_func(&"foo", vec![], vec![t.clone()],vec![
                    Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![
                            return_stmt(vec![l.clone()])
                        ],
                        elif_branches: vec![],
                        else_branch: None,
                        span: span(),
                    })
                ]);

                let result = return_branch_analysis_hazmat_wrapper(&dummy_func);

                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Expected function `foo` to return, but we found no return statements"));
            }
        }
    }


    // Same as above test, but since main if branch is empty, this should always panic.
    #[should_panic]
    #[test]
    fn if_statement_empty_panics() {
        for t in ALL_TYPES_WITH_DYN_ARR.iter() {
            let dummy_func = returning_func(&"foo", vec![], vec![t.clone()],vec![
                Stmt::If(IfStmt{
                    condition: int32_lit(2),
                    if_branch: vec![],
                    elif_branches: vec![],
                    else_branch: None,
                    span: span(),
                })
            ]);

            let _ = return_branch_analysis_hazmat_wrapper(&dummy_func);
        }
    }



    // Same as above, but this time main branch contains return, but else branch is Some, but empty. (this should panic because
    // return_branch_analysis_hazmat_wrapper assumes all function branches contain at least 1 statement, which
    // is what is guaranteed by dead_code_analysis.)
    #[should_panic]
    #[test]
    fn if_statement_returns_else_branch_empty_panics() {
        for t in ALL_TYPES_WITH_DYN_ARR.iter() {
            let dummy_func = returning_func(&"foo", vec![], vec![t.clone()],vec![
                Stmt::If(IfStmt{
                    condition: int32_lit(1),
                    if_branch: vec![
                        return_stmt(vec![int32_lit(2)])
                    ],
                    elif_branches: vec![],
                    else_branch: Some(vec![]),
                    span: span(),
                })
            ]);

            let _ = return_branch_analysis_hazmat_wrapper(&dummy_func);
        }
    }

    // Same as above, but this time main branch empty, and else branch returns. (this should panic because
    // return_branch_analysis_hazmat_wrapper assumes all function branches contain at least 1 statement, which
    // is what is guaranteed by dead_code_analysis.)
    #[should_panic]
    #[test]
    fn empty_if_statement_else_branch_returns_panics() {
        for t in ALL_TYPES_WITH_DYN_ARR.iter() {
            let dummy_func = returning_func(&"foo", vec![], vec![t.clone()],vec![
                Stmt::If(IfStmt{
                    condition: int32_lit(1),
                    if_branch: vec![],
                    elif_branches: vec![],
                    else_branch: Some(vec![
                        return_stmt(vec![int32_lit(2)])
                    ]),
                    span: span(),
                })
            ]);        

            let _ = return_branch_analysis_hazmat_wrapper(&dummy_func);
        }
    }

    // If statement with an else branch will always have one branch execute, but main branch does not return, meaning it can't always return,
    // 
    #[test]
    fn if_statement_not_return_with_else_branch_return_errors() {
        let literals = get_all_literals_with_var_and_var_arr();
        for l in literals {
            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                let dummy_func = returning_func(&"foo", vec![], vec![t.clone()],vec![
                    Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![
                            Stmt::Expr(l.clone())
                        ],
                        elif_branches: vec![],
                        else_branch: Some(vec![
                            return_stmt(vec![l.clone()])
                        ]),
                        span: span(),
                    })
                ]);

                let result = return_branch_analysis_hazmat_wrapper(&dummy_func);

                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Expected function `foo` to return, but we found no return statements"));
            }
        }
    }


    // If statement with an else branch will always have one branch execute, but else branch does not return, meaning it can't always return,
    // 
    #[test]
    fn if_statement_return_with_else_branch_not_returns_errors() {
        let literals = get_all_literals_with_var_and_var_arr();
        for l in literals {
            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                let dummy_func = returning_func(&"foo", vec![], vec![t.clone()],vec![
                    Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![
                            return_stmt(vec![l.clone()])
                        ],
                        elif_branches: vec![],
                        else_branch: Some(vec![
                            Stmt::Expr(l.clone())
                        ]),
                        span: span(),
                    })
                ]);

                let result = return_branch_analysis_hazmat_wrapper(&dummy_func);

                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Expected function `foo` to return, but we found no return statements"))              
            }
        }
    }
    

    // If statement with an else branch will always have one branch execute, and if both branches return, it can always return,
    // 
    #[test]
    fn if_statement_with_else_branch_both_return() {
        let literals = get_all_literals_with_var_and_var_arr();
        for l in literals {
            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                let dummy_func = returning_func(&"foo", vec![], vec![t.clone()],vec![
                    Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![
                            return_stmt(vec![l.clone()])
                        ],
                        elif_branches: vec![],
                        else_branch: Some(vec![
                            return_stmt(vec![l.clone()])
                        ]),
                        span: span(),
                    })
                ]);

                let result = return_branch_analysis_hazmat_wrapper(&dummy_func);

                assert!(result.is_ok());
            }
        }
    }

    // If statement with elif branch, and an else branch will always have one branch execute, and if even one branch not returns, 
    // it cant gurantee that it will always return.
    #[test]
    fn if_statement_with_else_branch_return_elif_branch_not_return_error() {
        let literals = get_all_literals_with_var_and_var_arr();
        for l in literals {
            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                let dummy_func = returning_func(&"foo", vec![], vec![t.clone()],vec![
                    Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![
                            return_stmt(vec![l.clone()])
                        ],
                        elif_branches: vec![(l.clone(), vec![
                            Stmt::Expr(l.clone())
                        ])],
                        else_branch: Some(vec![
                            return_stmt(vec![l.clone()])
                        ]),
                        span: span(),
                    })
                ]);
                
                let result = return_branch_analysis_hazmat_wrapper(&dummy_func);

                assert!(result.is_err());
                println!("hi {:?}", result);
                assert!(result.unwrap_err().to_string().contains("Expected function `foo` to return, but we found no return statements"))
            }
        }
    }

    // If statement with elif branch, and an else branch will always have one branch execute, and if even one branch not returns, 
    // it cant gurantee that it will always return.
    // 
    #[test]
    fn if_statement_with_elif_branch_return_else_branch_not_return_error() {
        let literals = get_all_literals_with_var_and_var_arr();
        for l in literals {
            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                let dummy_func = returning_func(&"foo", vec![], vec![t.clone()],vec![
                    Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![
                            return_stmt(vec![l.clone()])
                        ],
                        elif_branches: vec![(l.clone(), vec![
                            return_stmt(vec![l.clone()])
                        ])],
                        else_branch: Some(vec![
                            Stmt::Expr(l.clone())
                        ]),
                        span: span(),
                    })
                ]);

                let result = return_branch_analysis_hazmat_wrapper(&dummy_func);

                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Expected function `foo` to return, but we found no return statements"));
            }
        }
    }



    // If statement with elif branch, and an else branch will always have one branch execute, and if even one branch not returns, 
    // it cant gurantee that it will always return.
    // 
    #[test]
    fn if_statement_not_return_with_elif_branch_and_else_branch_return_error() {
        let literals = get_all_literals_with_var_and_var_arr();
        for l in literals {
            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                let dummy_func = returning_func(&"foo", vec![], vec![t.clone()],vec![
                    Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![
                            Stmt::Expr(l.clone())
                        ],
                        elif_branches: vec![(l.clone(), vec![
                            return_stmt(vec![l.clone()])
                        ])],
                        else_branch: Some(vec![
                            return_stmt(vec![l.clone()])
                        ]),
                        span: span(),
                    })
                ]);

                let result = return_branch_analysis_hazmat_wrapper(&dummy_func);

                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Expected function `foo` to return, but we found no return statements"));
            }
        }
    }

    // If statements within infinite loops without an else branch, returning only in one branch (main branch), 
    // even though the statement it may or may not execute, that is fine. because the if statement
    // is not the last statement of function, the infinite loop is.
    // 
    #[test]
    fn infinite_if_statement_returns() {
        let literals = get_all_literals_with_var_and_var_arr();
        for l in literals {
            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                let dummy_func = returning_func(&"foo", vec![], vec![t.clone()],vec![
                    Stmt::Infinite(InfiniteStmt{
                        branch: vec![
                            Stmt::If(IfStmt{
                                condition: l.clone(),
                                if_branch: vec![
                                    return_stmt(vec![l.clone()])
                                ],
                                elif_branches: vec![],
                                else_branch: None,
                                span: span(),
                            })
                        ],
                        span: span()
                    })]);


                let result = return_branch_analysis_hazmat_wrapper(&dummy_func);

                assert!(result.is_ok());
            }
        }
    }

    #[test]
    fn infinite_if_statement_returns_with_else_branch_returns() {
        let literals = get_all_literals_with_var_and_var_arr();
        for l in literals {
            for t in ALL_TYPES_WITH_DYN_ARR.iter() {               
                let dummy_func = returning_func(&"foo", vec![], vec![t.clone()],vec![
                    Stmt::Infinite(InfiniteStmt{
                        branch: vec![
                            Stmt::If(IfStmt{
                                condition: l.clone(),
                                if_branch: vec![
                                    return_stmt(vec![l.clone()])
                                ],
                                elif_branches: vec![],
                                else_branch: Some(vec![
                                    return_stmt(vec![l.clone()])
                                ]),
                                span: span(),
                            })
                        ],
                        span: span()
                    })]);
                    
                let result = return_branch_analysis_hazmat_wrapper(&dummy_func);

                assert!(result.is_ok());
            }
        }
    }


    #[test]
    fn infinite_if_statement_returns_with_elif_branch_returns() {
        let literals = get_all_literals_with_var_and_var_arr();
        for l in literals {
            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                let dummy_func = returning_func(&"foo", vec![], vec![t.clone()],vec![
                    Stmt::Infinite(InfiniteStmt{
                        branch: vec![
                            Stmt::If(IfStmt{
                                condition: l.clone(),
                                if_branch: vec![
                                    return_stmt(vec![l.clone()])
                                ],
                                elif_branches: vec![(l.clone(), vec![

                                    return_stmt(vec![l.clone()])
                                ])],
                                else_branch: None,
                                span: span(),
                            })
                        ],
                        span: span()
                    })]);

                let result = return_branch_analysis_hazmat_wrapper(&dummy_func);

                assert!(result.is_ok());
            }
        }
    }


    #[test]
    fn infinite_if_statement_returns_with_elif_branch_returns_else_branch_retirns() {
        let literals = get_all_literals_with_var_and_var_arr();
        for l in literals {
            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                let dummy_func = returning_func(&"foo", vec![], vec![t.clone()],vec![
                    Stmt::Infinite(InfiniteStmt{
                        branch: vec![
                            Stmt::If(IfStmt{
                                condition: l.clone(),
                                if_branch: vec![
                                    return_stmt(vec![l.clone()])
                                ],
                                elif_branches: vec![(l.clone(), vec![
                                    return_stmt(vec![l.clone()])
                                ])],
                                else_branch: Some(vec![
                                    return_stmt(vec![l.clone()])
                                ]),
                                span: span(),
                            })
                        ],
                        span: span()
                    })]);

                let result = return_branch_analysis_hazmat_wrapper(&dummy_func);

                assert!(result.is_ok());
            }
        }
    }
}