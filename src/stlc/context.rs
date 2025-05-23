use crate::stlc::ast::{Binding, TType, Term, TIndex};

pub struct Context(Vec<(String, Binding)>);

impl Context {
    pub fn new() -> Self {
        Context(Vec::new())
    }

    pub fn add_binding(&mut self, name: &str, binding: Binding) -> TIndex {
        self.0.push((name.to_string(), binding));
        return self.0.len() - 1
    }

    pub fn get_binding_from_idx(&self, index: usize) -> Binding {
        let (_, binding) = &self.0[index];
        return binding.clone();
    }

    pub fn get_type_from_idx(&self, index: usize) -> Option<TType> {
        let (_, binding) = &self.0[index];
        match binding {
            Binding::NameBinding => None,
            Binding::VariableBinding(ttype) => return Some(ttype.clone()),
        }
    }

    pub fn get_name_from_idx(&self, index: usize) -> String {
        let Context(bindings) = self;
        let (name, _) = &bindings[index];
        return name.clone();
    }

    pub fn get_idx_from_name(&self, name: &str) -> Option<usize> {
        let Context(bindings) = self;
        for (idx, (name2, _)) in bindings.iter().enumerate() {
            if name2 == name {
                return Some(idx);
            }
        }
        return None;
    }

    pub fn check_type(&self, term: &Term) -> Result<TType, &str> {
        match term {
            Term::TTrue => Ok(TType::TBool),
            Term::TFalse => Ok(TType::TBool),
            Term::TVariable(index) => match self.get_type_from_idx(*index) {
                None => Err("No such type"),
                Some(ttype) => Ok(ttype),
            },
            Term::TConditional(ifs) => {
                let (condition_term, then_term, else_term) = *ifs.to_owned();
                let Ok(condition_type) = self.check_type(&condition_term) else {
                    return Err("Bad condition type");
                };
                let Ok(then_type) = self.check_type(&then_term) else {
                    return Err("Bad then type");
                };
                let Ok(else_type) = self.check_type(&else_term) else {
                    return Err("Bad else type");
                };

                match (condition_type, then_type, else_type) {
                    (TType::TBool, a, b) => {
                        if a == b {
                            Ok(a)
                        } else {
                            Err("Diverging branches!")
                        }
                    }
                    _ => Err("Condition is not Bool!"),
                }
            }
            Term::TAbstraction(abs) => {
                let (variable_name, term) = abs.to_owned();
                let Ok(term_type) = self.check_type(&term) else {
                    return Err("Bad abstraction type!");
                };
                let Some(index) = self.get_idx_from_name(&variable_name) else {
                    return Err("No such binding found!");
                };
                let Some(variable_type) = self.get_type_from_idx(index) else {
                    return Err("Bad type");
                };
                Ok(TType::TFun(Box::new((variable_type, term_type))))
            }
            Term::TApplication(app) => {
                let (lambda, arg) = *app.to_owned();
                let Ok(lambda_type) = self.check_type(&lambda) else {
                    return Err("Bad lambda type!");
                };
                let Ok(arg_type) = self.check_type(&arg) else {
                    return Err("Bad argument type");
                };
                match lambda_type {
                    TType::TFun(fun) => {
                        let (from, to) = *fun;
                        if from == arg_type {
                            Ok(to)
                        } else {
                            Err("Lambda argument mismatch")
                        }
                    }
                    _ => Err("Wrong lambda type"),
                }
            }
        }
    }
}


