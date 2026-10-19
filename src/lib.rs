pub mod parser;
pub mod semantic;
pub mod transpiler;
pub mod compile;
pub mod error;
pub mod consts;
pub mod ast;
#[cfg(test)]
mod tests_consts;
#[cfg(test)]
mod semantic_test_helpers;


pub enum CompileInfo {
    CompileTo(String),
    DoNotCompile
}

///
/// # Panics
/// Panics if there are parsig errors, semantic errors, or compile errors.
///
#[expect(clippy::must_use_candidate, reason = "I dont want force ppl to use return value from this function")]
pub fn compile_goldlang_src(source: &str, compile_info: CompileInfo) -> String {
    // Parse source code
    let mut ast = parser::parse(source).expect("Parsing failed");

    println!("Pure AST: {ast:#?}\n\n\n");

    // Ensures that the source code contains a `main` function
    //
    // The reason this check is here, and not in semantics layer, is because if I were to add it to
    // semantics layer, all unit tests for semantics analysis layer would require a main function
    // and im frankly too lazy to redo them, and honestly, this isn't too too bad ;)
    //
    assert!(ast.functions.iter().any(|f| f.name == "main"), "Missing `main` function");

    // Run semantic checks, enforce language rules, modify AST to remove inferred and replace with explicit types, etc.
    semantic::check_semantics(&mut ast).expect("Semantic errors");
    println!("Checked AST: {ast:#?}\n\n\n");


    // Transpile to Rust code
    let rust_code = transpiler::transpile(&ast);
    println!("Transpiled Rust code:\n{rust_code:#}\n");

    match compile_info {
        CompileInfo::CompileTo(target_path) => {
            // Compile the transpiled Rust code into target_path.
            compile::compile(&rust_code, &target_path);
        },

        CompileInfo::DoNotCompile => {}
    }

    rust_code
}

