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
//(false)(fun x -> if x then false else true)

pub fn parse_ast(pair: Pair<Rule>, context: &mut Context) -> Option<Term> {
    match pair.as_rule() {
        Rule::term => {
                let inner_pair = pair.into_inner().next().unwrap();
                parse_ast(inner_pair, context)
            },
        Rule::bool => {
                match pair.as_str() {
                    "true" => Some(Term::TTrue),
                    "false" => Some(Term::TFalse),
                    _ => unreachable!("Rule bool contains only two keywords")
                }
            },
        Rule::lambda => {
                let inner = pair.into_inner();
                let mut param_name = "";
                let mut body = None; //Placeholder
                for p in inner.into_iter() {
                    match p.as_rule() {
                        Rule::WHITESPACE | Rule::arrow => continue,
                        Rule::variable_name => param_name = p.as_str().trim(),
                        Rule::term => body = parse_ast(p.clone(), context),
                        _ => unreachable!("Rule lambda has only name, term and arrow")
                    }
                }
                Some(Term::TAbstraction((param_name.to_string(), Box::new(body.unwrap()))))
            },
        Rule::application => {
                let inner = pair.into_inner();
                let mut terms = Vec::new();
                for p in inner.into_iter() {
                    match p.as_rule() {
                        Rule::WHITESPACE => continue,
                        Rule::term => { terms.push(parse_ast(p.clone(), context).expect("could not parse")) }
                        _ => unreachable!("Rule application consists only of whitespace and two terms")
                    }
                }
                assert_eq!(terms.len(), 2);
                Some(Term::TApplication(Box::new((terms[0].clone(), terms[1].clone()))))
            }
        Rule::declaration => {
            let inner = pair.into_inner();
            //let mut name = None;
            let mut typename = "";
            for p in inner.into_iter() {
                match p.as_rule() {
                    Rule::WHITESPACE => continue,
                    Rule::typename => typename = p.as_str(),
                    Rule::variable_name => {
                        match typename {
                            "Bool" => {
                                let index = context.add_binding(p.as_str(), Binding::VariableBinding(TType::TBool));
                                return Some(Term::TVariable(index));
                            }
                            _ => panic!("Only booleans supported"),

                        }
                    },
                    _ => unreachable!()
                }
            }
            return None

        },
        Rule::conditional => {
            let inner = pair.into_inner();
            let mut terms = Vec::new();
            let mut count_keywords = 0;
            for p in inner.into_iter() {
                 
                match p.as_rule() {
                    Rule::keywords => {
                        count_keywords += 1;
                        match count_keywords {
                            1 => assert_eq!(p.as_str(), "if"),
                            2 => assert_eq!(p.as_str(), "then"),
                            3 => assert_eq!(p.as_str(), "else"),
                            _ => unreachable!()
                        }
                    },
                    Rule::term => {
                        let try_parse = parse_ast(p.clone(), context);
                        match try_parse {
                            Some(parsed_term) => terms.push(parsed_term),
                            None => return None
                        }
                    }
                    _ => unreachable!()
                }
                assert_eq!(terms.len(), 3);
                return Some(
                    Term::TConditional(
                        Box::new(
                            (terms[0].clone(), terms[1].clone(), terms[2].clone())
                            )
                        )
                    );
            }
            None
        },
        _ => unreachable!()
    }
}

#[test]
fn test1() {
    if let Ok(term) = STLCParser::parse(Rule::term, "fun x -> (fun y -> true) ") {
        let mut context = Context::new();
        println!("{:?}", parse_ast(term.into_iter().next().unwrap(), &mut context));
    };
}


#[test]
fn test2() {
    if let Ok(term) = STLCParser::parse(Rule::term, "fun x -> false ") {
        let mut context = Context::new();
        println!("{:?}", parse_ast(term.into_iter().next().unwrap(), &mut context));
    };
}

