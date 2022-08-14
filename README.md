# LR Language [![Build Status](https://travis-ci.com/kgrech/lr-lang.svg?branch=main)](https://app.travis-ci.com/github/kgrech/lr-lang)

Implementation of small programming language based on LR(1) grammar parser using [Rust](https://www.rust-lang.org/) 
and [Lalrpop](https://github.com/lalrpop/lalrpop) crate. 
Written for learning purposes.

Read the tutorial on dev-to:
- [Part I: a bit of boring theory](https://dev.to/kgrech/writing-a-new-programming-language-part-i-a-bit-of-boring-theory-65e)
- [Part II: Variables and expressions](https://dev.to/kgrech/writing-a-new-programming-language-part-ii-variables-and-expressions-hh4)
- [Part III: Type System](https://dev.to/kgrech/writing-a-new-programming-language-part-iii-type-system-2ji3)
- [Part IV: Boolean expressions and if statements](https://dev.to/kgrech/writing-a-new-programming-language-part-iv-boolean-expressions-and-if-statements-35d8)


## Run simple program
```
cargo run -- -p test_programs/simple_calculator.lrlang
```
## Run tests
```
cargo test
```
