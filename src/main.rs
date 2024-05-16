use std::collections::HashMap;
use std::io;

mod parser;

fn main() {
    let mut input = String::new();
    println!("Please enter the knowledge base (formulas separated by a comma):");
    io::stdin().read_line(&mut input).unwrap();
    let formulas: Vec<&str> = input.trim().split(',').collect();
    
    let mut knowledge_base = Vec::new();
    for formula in formulas {
        let tokens = parser::lex(formula.trim());
        let mut parser = parser::Parser::new(tokens);
        let ast = parser.parse().expect("Syntax error");
        knowledge_base.push(ast);
    }

    println!("Please enter the formula to check:");
    let mut alpha = String::new();
    io::stdin().read_line(&mut alpha).unwrap();
    let tokens = parser::lex(alpha.trim());
    let mut parser = parser::Parser::new(tokens);
    let alpha_ast = parser.parse().expect("Syntax error");

    let mut atoms = HashMap::new();
    for ast in &knowledge_base {
        find_atoms(ast, &mut atoms);
    }
    find_atoms(&alpha_ast, &mut atoms);

    let mut truth_values = vec![HashMap::new(); 1 << atoms.len()];
    generate_truth_values(&mut truth_values, &atoms);

    let mut valid = true;
    for truth_assignment in &truth_values {
        if knowledge_base.iter().all(|ast| evaluate(ast, truth_assignment)) {
            if !evaluate(&alpha_ast, truth_assignment) {
                valid = false;
                break;
            }
        }
    }

    if valid {
        println!("The formula is a logical consequence of the knowledge base (KB |= α).");
    } else {
        println!("The formula is NOT a logical consequence of the knowledge base (KB |≠ α).");
    }
}

// Trouve tout les atomes de l'expression
fn find_atoms(ast: &parser::AstNode, atoms: &mut HashMap<String, ()>) {
    match ast {
        parser::AstNode::Atom(atom) => {
            atoms.insert(atom.clone(), ());
        }
        parser::AstNode::Not(sub_ast) => {
            find_atoms(sub_ast, atoms);
        }
        parser::AstNode::And(left, right)
        | parser::AstNode::Or(left, right)
        | parser::AstNode::If(left, right)
        | parser::AstNode::Iff(left, right) => {
            find_atoms(left, atoms);
            find_atoms(right, atoms);
        }
    }
}

// Genere les valeurs de verites pour chaque combinaison
fn generate_truth_values(truth_values: &mut Vec<HashMap<String, bool>>, atoms: &HashMap<String, ()>) {
    let mut index = 0; // Initialise un index de position dans le vecteur de valeurs de vérité.
    let atom_names: Vec<String> = atoms.keys().cloned().collect(); // Récupère les noms des atomes

    // Parcourt toutes les combinaisons possibles de valeurs de vérité pour les atomes.
    for combination in 0..(1 << atoms.len()) {
        // Pour chaque combinaison de valeurs de vérité, on stock les valeurs de vérité de chaque atome.
        for (i, atom_name) in atom_names.iter().enumerate() {
            // Calcul de la valeur de vérité de l'atome dans cette combinaison.
            let truth_value = (combination & (1 << i)) != 0;
            // Insère la valeur de vérité de l'atome dans la combinaison.
            truth_values[index].insert(atom_name.clone(), truth_value);
        }
        index += 1; // Passe à la prochaine combinaison de valeurs de vérité.
    }
}

// Evalue si une expression est satisfaisable selon les valeurs de verite des atomes
fn evaluate(ast: &parser::AstNode, truth_assignment: &HashMap<String, bool>) -> bool {
    match ast {
        parser::AstNode::Atom(atom) => *truth_assignment.get(atom).unwrap_or(&false),
        parser::AstNode::Not(sub_ast) => !evaluate(sub_ast, truth_assignment), 
        parser::AstNode::And(left, right) => {
            evaluate(left, truth_assignment) && evaluate(right, truth_assignment)
        }
        parser::AstNode::Or(left, right) => {
            evaluate(left, truth_assignment) || evaluate(right, truth_assignment) 
        }
        parser::AstNode::If(left, right) => {
            !evaluate(left, truth_assignment) || evaluate(right, truth_assignment) 
        }
        parser::AstNode::Iff(left, right) => {
            let left_result = evaluate(left, truth_assignment);
            let right_result = evaluate(right, truth_assignment);
            (!left_result || right_result) && (left_result || !right_result)
        }
    }
}


