use std::io::{self, Write};
use programming_language::{module::Module, types::{type_of, empty_ctx}};

fn main() {
    let mut module = Module::new_with_prelude();

    loop {
        print!("🐈 ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() { break; }
        let line = line.trim();
        if line.is_empty() { continue; }

        // REPL commands
        if line.starts_with(":") {
            if let Some(filename) = line.strip_prefix(":load ") {
                match std::fs::read_to_string(filename.trim()) {
                    Ok(content) => match programming_language::parser::parse_module(&content) {
                        Ok((_, new_module)) => {
                            module = new_module;
                            
                            // PRIORITIZE "main" OR "this" OR last declaration
                            let ast_to_eval  = module.iter()
                                .find(|(name, _)| name == "main" || name == "this")
                                .map(|(_, ast)| ast)
                                .or_else(|| {
                                    module.iter().last().map(|(_, ast)| ast)
                                });

                            match ast_to_eval {
                                Some(ast_ref) => {  // ast_ref is &AST
                                    let term = ast_ref.clone().desugar(&module);  // &AST → Term ✓
                                    let mut ctx = empty_ctx();
                                    
                                    match type_of(&term, &mut ctx) {
                                        Ok(ty) => println!("Type: {:?}", ty),
                                        Err(e) => eprintln!("Type error: {:?}", e),
                                    }
                                    
                                    let mut t = term;
                                    let mut steps = 0;
                                    while let Some(next) = t.step() {
                                        t = next;
                                        steps += 1;
                                        if steps >= 1000 {
                                            println!("(Still evaluating...)");
                                            break;
                                        }
                                    }
                                    println!(" {}", t);
                                }
                                None => println!("No main/this/last expression found"),
                            }
                        }
                        Err(e) => eprintln!("Parse error: {:?}", e),
                    },
                    Err(e) => eprintln!("File error: {:?}", e),
                }
            } else if line == ":quit" || line == ":exit" {
                break;
            } else if line == ":env" {
                for (name, ast) in module.iter() {
                    println!("{name} = {:?}", ast);
                }
            } else if line == ":help" {
                println!("Commands:");
                println!("  :load <file>    Load & evaluate main/this/last");
                println!("  :env            Show declarations");
                println!("  :quit           Exit");
            } else {
                eprintln!("Unknown command: {}", line);
            }
            continue;
        }

        // REPL expression evaluation (unchanged)
        match programming_language::parser::parse_ast(&module, line) {
            Ok((_, ast)) => {
                let term = ast.desugar(&module);
                let mut ctx = empty_ctx();
                
                match type_of(&term, &mut ctx) {
                    Ok(ty) => println!("Type: {:?}", ty),
                    Err(e) => {
                        eprintln!("Type error: {:?}", e);
                        continue;
                    }
                }
                
                let mut t = term;
                let mut steps = 0;
                while let Some(next) = t.step() {
                    t = next;
                    steps += 1;
                    if steps >= 1000 {
                        println!("(Still evaluating...)");
                        break;
                    }
                }
                println!("{}", t);
            }
            Err(e) => eprintln!("Parse error: {:?}", e),
        }
    }
}