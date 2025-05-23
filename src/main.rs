mod stlc;

use stlc::ast::{TermBuilder, TType};
use stlc::context::Context;

fn main() {

    let mut context = Context::new();
    let term1 = TermBuilder::new().term_true();
    let mut builder1 = TermBuilder::new();
    let func1 = builder1.var_typed(&mut context, TType::TBool, "x");
    let ttrue = builder1.term_true();
    let tfalse = builder1.term_false();
    let term4 = builder1.conditional(func1.clone(), ttrue, tfalse);
    let func2 = builder1.abstraction("x", term4.clone());


    println!("{:?}", context.check_type(&term1));
    println!("{:?}", context.check_type(&func1));
    println!("{:?}", context.check_type(&func2));
    println!("{:?}", context.check_type(&term4));
}

