use pest::error::Error;
use pest::{iterators::Pair, Parser, RuleType};
use pest_derive::Parser;


use crate::stlc::ast::{TType, Term, TermBuilder, Binding};
use crate::Context;


#[derive(Parser)]
#[grammar = "../grammars/stlc.pest"] // relative to src
struct STLCParser;

//x: Bool
//fun x -> (fun y -> y)
//fun x -> x 
//fun x -> true 
//fun x -> if x then false else true
//
//
pub fn parse() -> Result<Vec<Term>, Error<Rule>> {
    let mut ast = vec![];

    Ok(ast)
}

pub fn parse_ast(pair: Pair<Rule>, context: &mut Context) -> Option<Term> {

    match pair.as_rule() {
        Rule::term => {
            println!("Rule for term received: {:?}", pair.as_str());
            parse_ast(pair.into_inner().next()?, context)
        },
        Rule::lambda => {
            
            let mut variable_name = "";
            let mut inner_term = None;
            for inner_pair in pair.into_inner() {
                match inner_pair.as_rule() {
                    Rule::variable_name => {
                        variable_name = inner_pair.as_str();
                    },
                    Rule::term => {
                        inner_term = parse_ast(inner_pair, context);
                    },
                    _ => unreachable!()
                }
            }
            println!("Rule for lambda:\n - variable name: {:?}\n - inner_term: {:?}", variable_name, inner_term);
            let Some(parsed_term) = inner_term else {
                return None;
            };
            Some(Term::TAbstraction((variable_name.to_string(), Box::new(parsed_term))))


        }
        Rule::application_term => {
            for inner_pair in pair.into_inner() {
                println!("Application term: {:?}", inner_pair.as_str());
                if inner_pair.as_rule() == Rule::atom {
                    return parse_ast(inner_pair, context)
                }
            }
            None
        }
        Rule::conditional => todo!(),
        Rule::atom => {
            for inner_pair in pair.into_inner() {
                println!("atoms: {:?}", inner_pair.as_str());
                if inner_pair.as_rule() == Rule::bool {
                    return parse_ast(inner_pair, context)
                }
            }
            None
        }
        Rule::bool => {
            println!("Bool: '{}'", pair.as_str());
            match pair.as_str() {
                "true" => Some(Term::TTrue),
                "false" => Some(Term::TFalse),
                _ => None
            }
        },

        Rule::variable_name => unreachable!(),
        Rule::WHITESPACE => unreachable!(),
        Rule::base_type => unreachable!(),
        Rule::arrow_type => unreachable!(),
        Rule::typename => unreachable!(),
        Rule::keywords => unreachable!(),
        Rule::declaration => unreachable!(),
    }

}

#[test]
fn test1() {
    if let Ok(term) = STLCParser::parse(Rule::term, "fun x -> fun y -> true") {
        let mut context = Context::new();
        println!("Test1: {:?}", parse_ast(term.into_iter().next().unwrap(), &mut context));
    };
}

#[test]
fn test2() {
    if let Ok(term) = STLCParser::parse(Rule::term, "fun x -> fun y -> x") {
        let mut context = Context::new();
        println!("Test1: {:?}", parse_ast(term.into_iter().next().unwrap(), &mut context));
    };
}

#[test]
#[ignore]
fn test3() {
    if let Ok(term) = STLCParser::parse(Rule::term, "if false then true else false") {
        let mut context = Context::new();
        println!("Test2: {:?}", parse_ast(term.into_iter().next().unwrap(), &mut context));
    };
}
