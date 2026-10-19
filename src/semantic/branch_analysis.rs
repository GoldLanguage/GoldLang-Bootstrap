/// This file is mainly responsible for branch analysis, such as:
///     1. Analyzing functions for empty branchaes and erroring.
///
/// and
///     2. Analyzing branches for unreachable code (i.e. statements after a `return`, or a `break` statements)
///
/// and
///     3. Analyzing return branches to ensure branches correctly return, or infinitely loops
///        without breaking
///
/// P.S. No, these comments are not AI
///
use super::{
    Stmt,
    GoldError, 
    helpers, 
    Function
};


#[cfg(test)]
mod branch_analysis_tests; 


/// Performs code analysis on a specific function, the analysis include:
/// - empty branch analysis (ensuring the function body, and statements bodies, are not empty)
/// - unreachable code branch analysis (ensuring function body, and statements bodies, do not
/// contain unreachable statements and or branches)
/// - return branch analysis (ensuring functions with return signature always certainy returns regardless of branch)
///
pub fn code_analysis(
    func: &Function
) -> Result<(), GoldError> {
    if func.body.is_empty() {
        return Err(GoldError::Semantic(format!(
                    "Function `{}` has no statements, empty functions are not allowed! (line {} column {})",
                    func.name, func.span.line, func.span.column
                )))
    }
    empty_branch_analysis_hazmat(&func.body)?;

    unreachable_code_branch_analysis_hazmat(&func.body)?;

    if func.return_type.is_some() {
        return_branch_analysis_hazmat_wrapper(&func)?;
    }

    Ok(())
}


/// Checks statements in a statement block for empty branches.
/// NOTE: Do not call this function directly. this function only meant to be called within
/// `code_analysis`
fn empty_branch_analysis_hazmat(
    block: &Vec<Stmt>
) -> Result<(), GoldError> {
    assert!(!block.is_empty(), "(Compiler bug) Got an empty block of statements fed to `empty_branch_analysis_hazmat`.");

    for stmt in block {
        match stmt {
            Stmt::Return(_) |Stmt::Break(_) | Stmt::Continue(_) | Stmt::Lock(_) | Stmt::Unlock(_) | Stmt::Expr(_) | Stmt::VarDecl(_) 
                | Stmt::VarDeclMulti(_, _) | Stmt::VarAssign(_) | Stmt::VarAssignMulti(_) | Stmt::Const(_) => {},

            Stmt::Infinite(infinite_stmt) => {
                let body = &infinite_stmt.branch;
                if body.is_empty() {
                    return Err(GoldError::Semantic(format!(
                            "Infinite loop branch has no statements. Empty branches are not allowed (line {} column {})",
                            infinite_stmt.span.line, infinite_stmt.span.column
                        )))
                }

                empty_branch_analysis_hazmat(body)?;
            },
            Stmt::While(while_stmt) => {
                let body = &while_stmt.branch;
                if body.is_empty() {
                    return Err(GoldError::Semantic(format!(
                            "While loop branch has no statements. Empty branches are not allowed (line {} column {})",
                            while_stmt.span.line, while_stmt.span.column
                        )))
                }

                empty_branch_analysis_hazmat(body)?;
            },
            Stmt::For(for_stmt) => {
                let body = &for_stmt.branch;
                if body.is_empty() {
                    return Err(GoldError::Semantic(format!(
                            "For loop branch has no statements. Empty branches are not allowed (line {} column {})",
                            for_stmt.span.line, for_stmt.span.column
                        )))
                }

                empty_branch_analysis_hazmat(body)?;
            },
            Stmt::If(if_stmt) => {
                if if_stmt.if_branch.is_empty() {
                    return Err(GoldError::Semantic(format!(
                            "If statement `main` branch has no statements. Empty branches are not allowed (line {} column {})",
                            if_stmt.span.line, if_stmt.span.column
                        )))
                }

                empty_branch_analysis_hazmat(&if_stmt.if_branch)?;

                for s_vec in &if_stmt.elif_branches {
                    let expr_span = helpers::expr_span(&s_vec.0);

                    if s_vec.1.is_empty() {
                        return Err(GoldError::Semantic(format!(
                            "If statement `elif` branch has no statements. Empty branches are not allowed (line {} column {})",
                            expr_span.line, expr_span.column
                        )))
                    }

                    empty_branch_analysis_hazmat(&s_vec.1)?;
                }

                if let Some(else_branch) = &if_stmt.else_branch {
                    if else_branch.is_empty() {
                        return Err(GoldError::Semantic(format!(
                            "If statement `else` branch has no statements. Empty branches are not allowed (line {} column {})",
                            if_stmt.span.line, if_stmt.span.column
                        )))
                    } 
                
                    empty_branch_analysis_hazmat(else_branch)?;
                }
            }
        }
        
    }

    Ok(())
}


/// Performs unreachable code branch analysis, errors if it detects statements and or branches that could never be
/// "reached" (aka executed) due to `return`s and `break`s (or `continue`) statements.
///
/// NOTE: Do not call this function directly. this function only meant to be called within `code_analysis` function
///
fn unreachable_code_branch_analysis_hazmat(
    block: &Vec<Stmt>
) -> Result<(bool, bool), GoldError> {
    let mut certain_return_detected: bool = false;
    let mut certain_stop_detected: bool = false;

    for stmt in block {
        if certain_return_detected || certain_stop_detected {
            let current_stmt_span = helpers::stmt_span(stmt);
            let last_block_stmt_span = helpers::stmt_span(block.last().unwrap());

            if current_stmt_span == last_block_stmt_span {
                return Err(GoldError::Semantic(format!(
                            "Unreachable statement at line {}",
                            current_stmt_span.line
                        )))
            } else {
                return Err(GoldError::Semantic(format!(
                            "Unreachable code starting from line {} down to line {}",
                            current_stmt_span.line, last_block_stmt_span.line
                        )))
            }
        }

        match stmt {
            Stmt::Return(_) => certain_return_detected = true,
            Stmt::Break(_) | Stmt::Continue(_) => certain_stop_detected = true,
            Stmt::Infinite(inf_stmt) => (certain_return_detected, _) = unreachable_code_branch_analysis_hazmat(&inf_stmt.branch)?,
            Stmt::If(if_stmt) => if let Some(else_branch) = &if_stmt.else_branch {
                let (if_branch_returns, if_branch_stops) = unreachable_code_branch_analysis_hazmat(&if_stmt.if_branch)?;
                let (else_branch_returns, else_branch_stops) = unreachable_code_branch_analysis_hazmat(&else_branch)?;

                let (mut elif_branches_returns, mut elif_branches_stops) = (true, true);

                for s_vec in &if_stmt.elif_branches {
                    let (elif_returns, elif_stops) = unreachable_code_branch_analysis_hazmat(&s_vec.1)?;

                    if !elif_returns {
                        elif_branches_returns = false;
                    }

                    if !elif_stops {
                        elif_branches_stops = false;
                    }
                }

                certain_return_detected = if_branch_returns && elif_branches_returns && else_branch_returns;
                
                // If a specific branch returns, it might as well act as a stop (break or
                // continue) because whatever after it, is for sure unreachable. :)
                //
                certain_stop_detected = (if_branch_returns || if_branch_stops) && (elif_branches_returns || elif_branches_stops) && (else_branch_returns || else_branch_stops);

            // Without an else branch, we cannot guarantee any cetain return or stop, but we still need to catch errors.
            } else {
                unreachable_code_branch_analysis_hazmat(&if_stmt.if_branch)?;
                for s_vec in &if_stmt.elif_branches {
                    unreachable_code_branch_analysis_hazmat(&s_vec.1)?;
                }
            },
            
            Stmt::While(while_stmt) => (_, _) = unreachable_code_branch_analysis_hazmat(&while_stmt.branch)?,
            Stmt::For(for_stmt) => (_, _) = unreachable_code_branch_analysis_hazmat(&for_stmt.branch)?,

            // Non branching statements, so we safely ignore them
            // I added them manually here to ensure I dont fuckk up in future if I add a new
            // statement that contains branches
            Stmt::Const(_) | Stmt::Expr(_) | Stmt::VarDecl(_) | Stmt::VarDeclMulti(..) | Stmt::VarAssign(_) | Stmt::VarAssignMulti(..) |
            Stmt::Lock(_) | Stmt::Unlock(_) => {}
        }
        
    }
    
    Ok((certain_return_detected, certain_stop_detected))
}


/// Performs return branch analysis on a statement (the last statement in a block), to ensure 
/// that the statement (or its branch body) almost always certainly returns, or never returns.
///
/// This is a safe wrapper around `return_branch_analysis_hazmat`
///
/// NOTE: Do not call this function directly. this function only meant to be called within `code_analysis` function, 
/// and only after calling all other branch analysis functions such as empty branches and unreachable branches.
///
fn return_branch_analysis_hazmat_wrapper(
    func: &Function
) -> Result<(), GoldError> {
    assert!(func.return_type.is_some(), "Its caller responsiblity to ensure `return_branch_analysis_hazmat_wrapper` is only called on functions with declared type. There is likely a bug in your code. Func: {func:?}");

    fn return_branch_analysis_hazmat(
        last_stmt: &Stmt,
        depth: usize
    ) -> Result<bool, GoldError> {
        match last_stmt {
            Stmt::Return(_) => Ok(true),
            Stmt::Break(break_stmt) if depth == 1 => Err(GoldError::Semantic(format!("Cannot break out of infinite loop since it is the last statement in a returning function (line {} column {})", break_stmt.span.line, break_stmt.span.column))),
            Stmt::Infinite(inf_stmt) => {
                // We allow infinte loops to not return even in returning functions, as long as
                // there's no breaks.
                //
                return_branch_analysis_hazmat(&inf_stmt.branch.last().unwrap(), depth + 1)?;
                Ok(true)
            },
            Stmt::If(if_stmt) if let Some(else_branch) = &if_stmt.else_branch => {
                let if_branch_returns   = return_branch_analysis_hazmat(&if_stmt.if_branch.last().unwrap(), depth)?;
                let else_branch_returns = return_branch_analysis_hazmat(&else_branch.last().unwrap(), depth)?;

                let mut elif_branches_returns = true;

                for s_vec in &if_stmt.elif_branches {
                    if !return_branch_analysis_hazmat(&s_vec.1.last().unwrap(), depth)? {
                        elif_branches_returns = false;
                    }
                }

                Ok(if_branch_returns && elif_branches_returns && else_branch_returns)
            },
            // This one we can't guarantee a return, but we will still analyse it to detect errors (i.e. break in infinite loop body)
            Stmt::If(if_stmt) => {
                return_branch_analysis_hazmat(&if_stmt.if_branch.last().unwrap(), depth)?;
                for s_vec in &if_stmt.elif_branches {
                    return_branch_analysis_hazmat(&s_vec.1.last().unwrap(), depth)?;
                }
                Ok(false)
            },


            // These statements depend on specific expression values in order to execute, therefore they may not always execute (e.g.
            // while loops, for loops, etc)
            // so we cannot deduce for certain that they return
            //
            // We only error thoug if they are at last statement on their own, and not, let's
            // say, inside an infinite loop statement, in which cause they are fine to be.
            //
            Stmt::While(while_stmt) if depth == 0 => Err(GoldError::Semantic(format!("While loop statements may or may not execute at all, therefore it cannot be the last statement in a returning function (line {} column {})", while_stmt.span.line, while_stmt.span.column))),
        
            Stmt::For(for_stmt) if depth == 0 => Err(GoldError::Semantic(format!("For loop statement may or may not execute at all, therefore it cannot be the last statement in a returning function (line {} column {})", for_stmt.span.line, for_stmt.span.column))),


            // These statements don't have branches (or have failed the earlier guard checks), therefore, we can deduce for certain that they cannot return.
            //
            Stmt::While(_) | Stmt::For(_) | Stmt::Const(_) | Stmt::Expr(_) | Stmt::VarDecl(_) | Stmt::Continue(_) | Stmt::VarDeclMulti(..) | Stmt::VarAssign(_) | Stmt::VarAssignMulti(..) |
            Stmt::Lock(_) | Stmt::Unlock(_) | Stmt::Break(_) => Ok(false)
        }
    }


    let last_stmt = func.body.last().unwrap();

    // Depth starts at 0
    let certainly_returns = return_branch_analysis_hazmat(last_stmt, 0)?;
    if !certainly_returns {
        return Err(GoldError::Semantic(format!("Expected function `{}` to return, but we found no return statements. (line {} column {})", func.name, func.span.line, func.span.column)))
    }

    Ok(())
}
