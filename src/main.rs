use std::io;
use std::collections::HashSet;

mod clause_set;
use clause_set::ClauseSet;
use clause_set::Clause;

fn dpll(mut clauses: ClauseSet) -> bool {
  clauses.propagate();

  if clauses.empty_set() { return true; }
  if clauses.empty_clause() { return false; }

  let literal = clauses.select_literal();
  let unit_clause = Clause::new_unit(literal);
  clauses.add_clause(unit_clause);

  if dpll(clauses.clone()) {
    return true;
  } else {
    clauses.remove_last_clause();
    let inverse_unit = Clause::new_unit(-literal);
    clauses.add_clause(inverse_unit);
    return dpll(clauses);
  }
}

fn parse_dimacs() -> ClauseSet {
  let mut clauses = ClauseSet::new();
  let mut clauses_read = 0;
  let mut clause_count = 1; // It's at least 1

  while clauses_read < clause_count {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("read error");
    if input_line.len() == 0 || input_line.starts_with("c") {
      continue;
    }

    let tokens: Vec<&str> = input_line.split_whitespace().collect();
    if input_line.starts_with("p") {
      // 0 = p, 1 = cnf, 2 = literal count, 3 = clause count
      if tokens[1] != "cnf" {
        panic!("Only CNF supported!");
      }
      clause_count = tokens[3].parse().unwrap();
    } else {
      // Assume clause
      let mut literals: HashSet::<clause_set::Literal>
        = tokens.iter().map(|l| l.parse().unwrap()).collect();
      literals.remove(&0); // Zero is not a valid literal -- just used for end of line
      clauses.add_clause(Clause::new(literals));
      clauses_read += 1;
    }
  }
  clauses
}

fn main() {
  let clauses = parse_dimacs();
  if dpll(clauses) {
    println!("SATISFIABLE");
  } else {
    println!("UNSATISFIABLE");
  }
}
