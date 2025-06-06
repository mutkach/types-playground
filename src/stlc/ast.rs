use std::collections::HashMap;

use super::context::Context;

pub type TIndex = usize;

#[derive(Debug, Clone)]
pub enum Term {
    TTrue,
    //TLet(TIndex, Box<Term>),
    TFalse,
    TVariable(TIndex),
    TConditional(Box<(Term, Term, Term)>),
    TAbstraction((String, Box<Term>)),
    TApplication(Box<(Term, Term)>)
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TType {
    TBool,
    TFun(Box<(TType, TType)>),
}


#[derive(Debug, Clone)]
pub enum Binding {
    NameBinding,
    VariableBinding(TType),
}


pub struct TermBuilder {
    variables: HashMap<String, TIndex>,
    next_index: TIndex,
}


impl TermBuilder {
    pub fn new() -> Self {
        TermBuilder {
            variables: HashMap::new(),
            next_index: 0,
        }
    }

    pub fn term_true(&self) -> Term {
        Term::TTrue
    }

    pub fn term_false(&self) -> Term {
        Term::TFalse
    }

    pub fn var_typed(&mut self, context: &mut Context, _type: TType, name: &str) -> Term {
        context.add_binding(name, Binding::VariableBinding(_type));
        self.variables.insert(name.to_string(), self.next_index);
        let idx = self.next_index;
        self.next_index += 1;
        Term::TVariable(idx)
    }

    pub fn var(&mut self, context: &mut Context, name: &str) -> Term {
        context.add_binding(name, Binding::NameBinding);
        self.variables.insert(name.to_string(), self.next_index);
        let idx = self.next_index;
        self.next_index += 1;
        Term::TVariable(idx)
    }

    pub fn application(&self, func: Term, arg: Term) -> Term {
        Term::TApplication(Box::new((func, arg)))
    }

    pub fn abstraction(&self, param: &str, body: Term) -> Term {
        Term::TAbstraction((param.to_string(), Box::new(body)))
    }

    pub fn conditional(&self, _if: Term, then: Term, _else: Term) -> Term {
        Term::TConditional(Box::new((_if, then, _else)))
    }

    pub fn lambda(&mut self, param: &str, body_fn: impl FnOnce(&mut Self) -> Term) -> Term {
        let body = body_fn(self);
        self.abstraction(param, body)
    }
}
