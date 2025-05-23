# Type Theory showcase

## Roadmap:
1. Implement Simply-Typed Lambda Calculus type-checking (on AST only).
2. Add parsing and .


### Examples:

```
    let mut context = Context::new(); // create context to track bindings
    let mut builder1 = TermBuilder::new(); // create helper for AST creation
    let func1 = builder1.var_typed(&mut context, TType::TBool, "x"); // λx.x
    let ttrue = builder1.term_true(); // TTerm::TTrue instance
    let tfalse = builder1.term_false(); // TTerm::TTrue instance
    let term4 = builder1.conditional(func1.clone(), ttrue, tfalse); // if "x" then true else false
    let func2 = builder1.abstraction("x", term4.clone()); // if λx.x then true else false


    println!("{:?}", context.check_type(&func1)); // **Bool**
    println!("{:?}", context.check_type(&term4)); // **Bool**
    println!("{:?}", context.check_type(&func2)); // **Bool -> Bool**
```
