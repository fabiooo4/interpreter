use core::panic;
use std::{collections::HashMap, rc::Rc};

use antlr_rust::{
    InputStream,
    common_token_stream::CommonTokenStream,
    parser_rule_context::BaseParserRuleContext,
    tree::{ErrorNode, ParseTree, ParseTreeVisitorCompat},
};

use crate::parser::{
    implexer::ImpLexer,
    impparser::{
        self, ADD, DIV, DeclarationContextAttrs, ImpParser, ImpParserContextType, MOD, MUL,
        MainContext, MainContextAttrs, MainContextExt, NegContext, NegContextAttrs, PowContext,
        Prec1opContext, Prec2opContext, SUB, ValContext, VarContextAttrs,
    },
    impvisitor::ImpVisitorCompat,
};

#[derive(Default)]
pub struct ImpInterpreter {
    pub res: f32,

    memory: HashMap<String, f32>,
}

impl ImpInterpreter {
    pub fn new() -> Self {
        ImpInterpreter::default()
    }

    pub fn parse(input: &str) -> Rc<BaseParserRuleContext<'_, MainContextExt<'_>>> {
        let input = InputStream::new(input.trim());

        // Create a TokenSource from the CharStream using the Imp grammar
        let lexer = ImpLexer::new(input);

        // Obtain the tokens from the TokenSource as a TokenStream
        let tokens = CommonTokenStream::new(lexer);

        // Create a parser that parses the Imp grammar
        let mut parser = ImpParser::new(tokens);

        // Execute the grammar from the 'main' nonterminal symbol
        parser.main().unwrap()
    }
}

impl ParseTreeVisitorCompat<'_> for ImpInterpreter {
    type Node = ImpParserContextType;
    type Return = f32;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.res
    }

    fn visit_error_node(&mut self, _node: &ErrorNode<'_, Self::Node>) -> Self::Return {
        panic!("Error encountered: {}", _node.symbol)
    }

    fn aggregate_results(&self, aggregate: Self::Return, next: Self::Return) -> Self::Return {
        aggregate + next
    }
}

impl ImpVisitorCompat<'_> for ImpInterpreter {
    fn visit_main(&mut self, ctx: &MainContext<'_>) -> Self::Return {
        self.visit(&*ctx.prog().unwrap())
    }

    //
    // Expressions
    //

    fn visit_val(&mut self, ctx: &ValContext<'_>) -> Self::Return {
        ctx.get_text()
            .parse()
            .expect("Failed to parse integer value")
    }

    fn visit_prec1op(&mut self, ctx: &Prec1opContext) -> Self::Return {
        let operator = ctx.op.to_owned().unwrap().token_type;

        let lhs = self.visit(
            &*ctx
                .lhs
                .clone()
                .expect("Failed to parse left operand"),
        );
        let rhs = self.visit(
            &*ctx
                .rhs
                .clone()
                .expect("Failed to parse right operand"),
        );

        match operator {
            MUL => lhs * rhs,
            DIV => lhs / rhs,
            MOD => lhs % rhs,
            _ => unreachable!(),
        }
    }

    fn visit_pow(&mut self, ctx: &PowContext<'_>) -> Self::Return {
        let lhs = self.visit(
            &*ctx
                .lhs
                .clone()
                .expect("Failed to parse left operand"),
        );
        let rhs = self.visit(
            &*ctx
                .rhs
                .clone()
                .expect("Failed to parse right operand"),
        );

        lhs.powf(rhs)
    }

    fn visit_prec2op(&mut self, ctx: &Prec2opContext<'_>) -> Self::Return {
        let operator = ctx.op.to_owned().unwrap().token_type;

        let lhs = self.visit(
            &*ctx
                .lhs
                .clone()
                .expect("Failed to parse left operand"),
        );
        let rhs = self.visit(
            &*ctx
                .rhs
                .clone()
                .expect("Failed to parse right operand"),
        );

        match operator {
            ADD => lhs + rhs,
            SUB => lhs - rhs,
            _ => unreachable!()
        }
    }

    fn visit_neg(&mut self, ctx: &NegContext<'_>) -> Self::Return {
        let val = self.visit(&*ctx.exp().unwrap());
        -val
    }

    //
    // Variables
    //

    fn visit_declaration(&mut self, ctx: &impparser::DeclarationContext<'_>) -> Self::Return {
        let var_name = ctx.VAR().unwrap().get_text();
        let val = self.visit(&*ctx.exp().unwrap());

        self.memory.insert(var_name, val);

        self.visit(&*ctx.decl().unwrap());

        0.
    }

    fn visit_var(&mut self, ctx: &impparser::VarContext<'_>) -> Self::Return {
        let var_name = ctx.VAR().unwrap().get_text();
        let val = self.memory.get(&var_name).unwrap_or(&0.);

        *val
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let program = "
        4+(4+4)
        ";

        let ast = ImpInterpreter::parse(program);
        let mut interpreter = ImpInterpreter::new();
        assert_eq!(12., interpreter.visit(&*ast))
    }

    #[test]
    fn test_sub() {
        let program = "
        4-(4-5)
        ";

        let ast = ImpInterpreter::parse(program);
        let mut interpreter = ImpInterpreter::new();
        assert_eq!(5., interpreter.visit(&*ast))
    }

    #[test]
    fn test_neg() {
        let program = "
        -(-4)
        ";

        let ast = ImpInterpreter::parse(program);
        let mut interpreter = ImpInterpreter::new();
        assert_eq!(4., interpreter.visit(&*ast))
    }

    #[test]
    fn test_mul() {
        let program = "
        2*(3*3)
        ";

        let ast = ImpInterpreter::parse(program);
        let mut interpreter = ImpInterpreter::new();
        assert_eq!(18., interpreter.visit(&*ast))
    }

    #[test]
    fn test_div() {
        let program = "
        (18/2)/2
        ";

        let ast = ImpInterpreter::parse(program);
        let mut interpreter = ImpInterpreter::new();
        assert_eq!(4.5, interpreter.visit(&*ast))
    }

    #[test]
    fn test_mod() {
        let program = "
        16 mod 2
        ";

        let ast = ImpInterpreter::parse(program);
        let mut interpreter = ImpInterpreter::new();
        assert_eq!(0., interpreter.visit(&*ast))
    }

    #[test]
    fn test_pow() {
        let program = "
        2^(2^3)
        ";

        let ast = ImpInterpreter::parse(program);
        let mut interpreter = ImpInterpreter::new();
        assert_eq!(256., interpreter.visit(&*ast))
    }

    #[test]
    fn test_precedence() {
        let program = "
         5-2*10/2
        ";

        let ast = ImpInterpreter::parse(program);
        let mut interpreter = ImpInterpreter::new();
        assert_eq!(-5., interpreter.visit(&*ast))
    }

    #[test]
    fn test_declaration() {
        let program = "
         a = 2; 0
        ";

        let ast = ImpInterpreter::parse(program);
        let mut interpreter = ImpInterpreter::new();
        let res = interpreter.visit(&*ast);
        assert_eq!(Some(&2.), interpreter.memory.get("a"));
        assert_eq!(0., res);
    }

    #[test]
    fn test_var_exp() {
        let program = "
         a = 2; a
        ";

        let ast = ImpInterpreter::parse(program);
        let mut interpreter = ImpInterpreter::new();
        let res = interpreter.visit(&*ast);
        assert_eq!(Some(&2.), interpreter.memory.get("a"));
        assert_eq!(2., res);
    }

    #[test]
    fn test_undeclared_var() {
        let program = "
         a
        ";

        let ast = ImpInterpreter::parse(program);
        let mut interpreter = ImpInterpreter::new();
        let res = interpreter.visit(&*ast);
        assert_eq!(None, interpreter.memory.get("a"));
        assert_eq!(0., res);
    }

    #[test]
    fn test_program() {
        let program = "
         base = 5;
         height = 10;
         base * height
        ";

        let ast = ImpInterpreter::parse(program);
        let mut interpreter = ImpInterpreter::new();
        let res = interpreter.visit(&*ast);
        assert_eq!(Some(&5.), interpreter.memory.get("base"));
        assert_eq!(Some(&10.), interpreter.memory.get("height"));
        assert_eq!(50., res);
    }
}
