use std::collections::HashSet;
use retain_mut::RetainMut;
use rand::seq::SliceRandom;

pub type Literal = i32;
pub type Variable = u16;

#[derive(Clone)]
pub struct Clause {
  pub literals: HashSet<Literal>
}

#[derive(PartialEq)]
enum Contains { Positively, Negatively, DoesNot }

impl Clause {
  pub fn new(literals: HashSet<Literal>) -> Clause {
    Clause {
      literals: literals,
    }
  }

  pub fn new_unit(literal: Literal) -> Clause {
    let mut literals = HashSet::new();
    literals.insert(literal);
    Clause {
      literals: literals,
    }
  }

  fn is_unit(&self) -> bool { self.literals.len() == 1 }

  fn contains_variable(&self, variable: Variable) -> Contains {
    let literal = variable as Literal;
    if self.literals.contains(&literal) {
      return Contains::Positively;
    } else if self.literals.contains(&-literal) {
      return Contains::Negatively;
    }
    return Contains::DoesNot;
  }
}

#[derive(Clone)]
pub struct ClauseSet {
  pub clauses: Vec<Clause>,
  empty_clause: bool
}

impl ClauseSet {
  pub fn new() -> ClauseSet {
    ClauseSet {
      clauses: vec![],
      empty_clause: false
    }
  }

  fn get_unit_clause(&self) -> Option<&Clause> {
    self.clauses.iter().find(|&clause| clause.is_unit())
  }

  pub fn add_clause(&mut self, clause: Clause) -> () {
    self.clauses.push(clause);
  }

  pub fn remove_last_clause(&mut self) -> () {
    self.clauses.pop();
  }

  pub fn propagate(&mut self) -> () {
    loop {
      if self.clauses.is_empty() {
        return;
      }
      if let Some(unit_clause) = self.get_unit_clause() {
        let unit = *unit_clause.literals.iter().next().unwrap();
        let variable = unit.abs() as Variable;
        let mut is_empty = false;
        self.clauses.retain_mut(|clause: &mut Clause| {
          let contains = clause.contains_variable(variable);
          if (contains == Contains::Positively && unit > 0)
            || (contains == Contains::Negatively && unit < 0)
          {
            return false;
          } else {
            if clause.is_unit() && clause.literals.contains(&-unit) {
              is_empty = true;
              return false;
            } else {
              if clause.literals.contains(&-unit) {
                clause.literals.remove(&-unit);
              }
              return true;
            }
          }
        });
        if is_empty {
          self.empty_clause = is_empty;
          break;
        }
      } else {
        return;
      }
    }
  }

  pub fn empty_set(&self) -> bool {
    !self.empty_clause && self.clauses.is_empty()
  }

  pub fn empty_clause(&self) -> bool {
    self.empty_clause
  }

  pub fn select_literal(&self) -> Literal {
    // I'm going to assume the first element in the clause is random enough
    // for ease (since I'm not too good with rust)
    *self.clauses
      .choose(&mut rand::thread_rng())
      .and_then(|clause| clause.literals.iter().next())
      .unwrap()
  }
}
