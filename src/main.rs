use std::collections::HashMap;
use std::io;

mod parser;

fn main() {
    let mut input = String::new();
    println!("Please enter a logical formula:");
    io::stdin().read_line(&mut input).unwrap();
    let tokens = parser::lex(input.trim());
    println!("Tokens: {:?}", tokens);
    
    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse().expect("Syntax error");
    
    // Trouver les atomes propositionnels dans la formule
    let mut atoms = HashMap::new();
    find_atoms(&ast, &mut atoms);
    println!("{:?}", ast);
    println!("{:?}",atoms);
    
    // Générer toutes les combinaisons possibles de valeurs de vérité pour les atomes
    let mut truth_values = vec![HashMap::new(); 1 << atoms.len()];
    generate_truth_values(&mut truth_values, &atoms);
    
    // Vérifier la satisfaisabilité de la formule pour chaque combinaison de valeurs de vérité
    let mut satisfiable_assignments = vec![];
    let mut unsatisfiable_assignments = vec![];
    for truth_assignment in &truth_values {
        if evaluate(&ast, truth_assignment) {
            satisfiable_assignments.push(truth_assignment.clone());
        } else {
            unsatisfiable_assignments.push(truth_assignment.clone());
        }
    }
    
    println!("Satisfiable assignments:");
    for assignment in &satisfiable_assignments {
        println!("{:?}", assignment);
    }
    
    println!("Unsatisfiable assignments:");
    for assignment in &unsatisfiable_assignments {
        println!("{:?}", assignment);
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


