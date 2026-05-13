mod panic_error_listener;
mod value;

use core::panic;
use std::str::FromStr;

use antlr_rust::{
    InputStream, Parser,
    common_token_stream::CommonTokenStream,
    tree::{ErrorNode, ParseTree, ParseTreeVisitorCompat},
};

use crate::{
    SyntaxTree,
    interpreter::{panic_error_listener::PanicErrorListener, value::Value},
    memory::{HashMemory, Memory},
    parser::{
        implexer::{ADD, AND, DIV, EQ, GE, GT, ImpLexer, LE, LT, MOD, MUL, NEQ, OR, SUB},
        impparser::{
            self, CastContextAttrs, DeclContextAttrs, IdContextAttrs, IfContextAttrs,
            IfElseContextAttrs, ImpParser, ImpParserContextType, IntContext, MainContext,
            MainContextAttrs, MutationContextAttrs, NegContextAttrs, NotContextAttrs,
            ParenContextAttrs, PrintContextAttrs, ToStrContextAttrs, WhileContextAttrs,
        },
        impvisitor::ImpVisitorCompat,
    },
    type_checker::value_type::Type,
};

#[derive(Default, Debug)]
pub struct ImpInterpreter {
    pub res: Value,

    memory: HashMemory<Value>,
}

impl ImpInterpreter {
    pub fn new() -> Self {
        ImpInterpreter::default()
    }

    pub fn parse(input: &str) -> SyntaxTree<'_> {
        let input = InputStream::new(input.trim());

        // Create a TokenSource from the CharStream using the Imp grammar
        let mut lexer = ImpLexer::new(input);
        lexer.remove_error_listeners();
        lexer.add_error_listener(Box::new(PanicErrorListener {}));

        // Obtain the tokens from the TokenSource as a TokenStream
        let tokens = CommonTokenStream::new(lexer);

        // Create a parser that parses the Imp grammar
        let mut parser = ImpParser::new(tokens);
        parser.remove_error_listeners();
        parser.add_error_listener(Box::new(PanicErrorListener {}));

        // Execute the grammar from the 'main' nonterminal symbol
        parser.main().unwrap()
    }

    pub fn interpret(&mut self, ast: &SyntaxTree) -> Value {
        self.visit(&**ast)
    }
}

impl ParseTreeVisitorCompat<'_> for ImpInterpreter {
    type Node = ImpParserContextType;
    type Return = Value;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.res
    }

    fn visit_error_node(&mut self, _node: &ErrorNode<'_, Self::Node>) -> Self::Return {
        panic!("Error encountered: {}", _node.symbol)
    }

    fn visit(
        &mut self,
        node: &<Self::Node as antlr_rust::parser::ParserNodeType<'_>>::Type,
    ) -> Self::Return {
        node.accept(self);

        // Set the final result in the interpreter
        self.res.clone()
    }
}

impl ImpVisitorCompat<'_> for ImpInterpreter {
    fn visit_main(&mut self, ctx: &MainContext<'_>) -> Self::Return {
        self.visit(&*ctx.prog().unwrap())
    }

    //
    // Types {
    //

    fn visit_int(&mut self, ctx: &IntContext<'_>) -> Self::Return {
        ctx.get_text().parse().unwrap_or_else(|e| panic!("{e}"))
    }

    fn visit_float(&mut self, ctx: &crate::parser::impparser::FloatContext<'_>) -> Self::Return {
        ctx.get_text().parse().unwrap_or_else(|e| panic!("{e}"))
    }

    fn visit_bool(&mut self, ctx: &crate::parser::impparser::BoolContext<'_>) -> Self::Return {
        ctx.get_text().parse().unwrap_or_else(|e| panic!("{e}"))
    }

    fn visit_string(&mut self, ctx: &crate::parser::impparser::StringContext<'_>) -> Self::Return {
        ctx.get_text().parse().unwrap_or_else(|e| panic!("{e}"))
    }

    fn visit_char(&mut self, ctx: &crate::parser::impparser::CharContext<'_>) -> Self::Return {
        ctx.get_text().parse().unwrap_or_else(|e| panic!("{e}"))
    }

    // } Types

    //
    // Arithmetic expressions {
    //

    fn visit_pow(&mut self, ctx: &crate::parser::impparser::PowContext<'_>) -> Self::Return {
        let base = self.visit(&*ctx.lhs.clone().unwrap());
        let exp = self.visit(&*ctx.rhs.clone().unwrap());

        base.pow(exp)
    }

    fn visit_mulDivMod(
        &mut self,
        ctx: &crate::parser::impparser::MulDivModContext<'_>,
    ) -> Self::Return {
        let operator = ctx.op.to_owned().unwrap().token_type;

        let lhs = self.visit(&*ctx.lhs.clone().unwrap());
        let rhs = self.visit(&*ctx.rhs.clone().unwrap());

        match operator {
            MUL => lhs * rhs,
            DIV => lhs / rhs,
            MOD => lhs % rhs,
            _ => unreachable!(),
        }
    }

    fn visit_addSub(&mut self, ctx: &impparser::AddSubContext<'_>) -> Self::Return {
        let operator = ctx.op.to_owned().unwrap().token_type;

        let lhs = self.visit(&*ctx.lhs.clone().unwrap());
        let rhs = self.visit(&*ctx.rhs.clone().unwrap());

        match operator {
            ADD => lhs + rhs,
            SUB => lhs - rhs,
            _ => unreachable!(),
        }
    }

    fn visit_neg(&mut self, ctx: &impparser::NegContext<'_>) -> Self::Return {
        -self.visit(&*ctx.exp().unwrap())
    }

    // } Arithmetic expressions

    //
    // Boolean expressione {
    //

    fn visit_not(&mut self, ctx: &crate::parser::impparser::NotContext<'_>) -> Self::Return {
        let val = self.visit(&*ctx.exp().unwrap());

        !val
    }

    fn visit_cmp(&mut self, ctx: &impparser::CmpContext<'_>) -> Self::Return {
        let operator = ctx.op.to_owned().unwrap().token_type;

        let lhs = self.visit(&*ctx.lhs.clone().unwrap());
        let rhs = self.visit(&*ctx.rhs.clone().unwrap());

        match operator {
            LT => Value::Bool(lhs < rhs),
            LE => Value::Bool(lhs <= rhs),
            GT => Value::Bool(lhs > rhs),
            GE => Value::Bool(lhs >= rhs),

            _ => unreachable!(),
        }
    }

    fn visit_eq(&mut self, ctx: &impparser::EqContext<'_>) -> Self::Return {
        let operator = ctx.op.to_owned().unwrap().token_type;

        let lhs = self.visit(&*ctx.lhs.clone().unwrap());
        let rhs = self.visit(&*ctx.rhs.clone().unwrap());

        match operator {
            EQ => Value::Bool(lhs == rhs),
            NEQ => Value::Bool(lhs != rhs),
            _ => unreachable!(),
        }
    }

    fn visit_andOr(&mut self, ctx: &impparser::AndOrContext<'_>) -> Self::Return {
        let operator = ctx.op.to_owned().unwrap().token_type;

        let lhs = self.visit(&*ctx.lhs.clone().unwrap());
        let rhs = self.visit(&*ctx.rhs.clone().unwrap());

        match operator {
            AND => lhs & rhs,
            OR => lhs | rhs,
            _ => unreachable!(),
        }
    }

    // } Boolean expressions

    //
    // Expressions {
    //
    fn visit_cast(&mut self, ctx: &impparser::CastContext<'_>) -> Self::Return {
        let exp = self.visit(&*ctx.exp().unwrap());
        let cast_typ = &*ctx.TYPE().unwrap().get_text();

        let cast_typ = Type::from_str(&ctx.TYPE().unwrap().get_text())
            .unwrap_or_else(|e| panic!("Failed to convert '{exp}' to '{cast_typ}': {e}"));

        exp.cast(cast_typ)
    }

    fn visit_paren(&mut self, ctx: &impparser::ParenContext<'_>) -> Self::Return {
        self.visit(&*ctx.exp().unwrap())
    }
    // } Expressions

    //
    // Variables {
    //

    fn visit_decl(&mut self, ctx: &impparser::DeclContext<'_>) -> Self::Return {
        let var_name = ctx.ID().unwrap().get_text();
        let val = self.visit(&*ctx.exp().unwrap());

        self.memory.add(var_name, val);

        Value::Void
    }

    fn visit_id(&mut self, ctx: &impparser::IdContext<'_>) -> Self::Return {
        let var_name = ctx.ID().unwrap().get_text();
        let val = self
            .memory
            .get(&var_name)
            .unwrap_or_else(|| panic!("Unknown variable {}", var_name));

        val.clone()
    }

    fn visit_mutation(&mut self, ctx: &impparser::MutationContext<'_>) -> Self::Return {
        let var_name = ctx.ID().unwrap().get_text();
        let exp = self.visit(&*ctx.exp().unwrap());
        self.memory
            .update(var_name.clone(), exp)
            .unwrap_or_else(|| panic!("Unknown variable {}", var_name));

        Value::Void
    }
    // } Variables

    //
    // Statements {
    //

    fn visit_print(&mut self, ctx: &impparser::PrintContext<'_>) -> Self::Return {
        let str = self.visit(&*ctx.exp().unwrap());
        println!("{}", str);

        Value::Void
    }

    fn visit_toStr(&mut self, ctx: &impparser::ToStrContext<'_>) -> Self::Return {
        let val = self.visit(&*ctx.exp().unwrap());

        Value::String(val.to_string())
    }

    fn visit_strConcat(&mut self, ctx: &impparser::StrConcatContext<'_>) -> Self::Return {
        let lhs = self.visit(&*ctx.lhs.clone().unwrap());
        let rhs = self.visit(&*ctx.rhs.clone().unwrap());

        Value::String(format!("{}{}", lhs, rhs))
    }

    fn visit_if(&mut self, ctx: &impparser::IfContext<'_>) -> Self::Return {
        let condition = self.visit(&*ctx.exp().unwrap());
        if let Value::Bool(true) = condition {
            // self.visit(&*ctx.stmt().unwrap())
            for node in ctx.stmt_all() {
                self.visit(&*node);
            }
        }

        Value::Void
    }

    fn visit_ifElse(&mut self, ctx: &impparser::IfElseContext<'_>) -> Self::Return {
        let condition = self.visit(&*ctx.exp().unwrap());
        if let Value::Bool(true) = condition {
            self.visit(&*ctx.true_branch.clone().unwrap())
        } else {
            self.visit(&*ctx.false_branch.clone().unwrap())
        }
    }

    fn visit_while(&mut self, ctx: &impparser::WhileContext<'_>) -> Self::Return {
        let mut condition = self.visit(&*ctx.exp().unwrap());
        while let Value::Bool(true) = condition {
            for node in ctx.stmt_all() {
                self.visit(&*node);
            }

            condition = self.visit(&*ctx.exp().unwrap());
        }

        Value::Void
    }

    // } Statements
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
        assert_eq!(Value::Int(12), interpreter.visit(&*ast))
    }

    #[test]
    fn test_sub() {
        let program = "
        4-(4-5)
        ";

        let ast = ImpInterpreter::parse(program);
        let mut interpreter = ImpInterpreter::new();
        assert_eq!(Value::Int(5), interpreter.visit(&*ast))
    }

    #[test]
    fn test_neg() {
        let program = "
        -(-4)
        ";

        let ast = ImpInterpreter::parse(program);
        let mut interpreter = ImpInterpreter::new();
        assert_eq!(Value::Int(4), interpreter.visit(&*ast))
    }

    #[test]
    fn test_mul() {
        let program = "
        2*(3*3)
        ";

        let ast = ImpInterpreter::parse(program);
        let mut interpreter = ImpInterpreter::new();
        assert_eq!(Value::Int(18), interpreter.visit(&*ast))
    }

    #[test]
    fn test_div() {
        let program = "
        (18/2)/2
        ";

        let ast = ImpInterpreter::parse(program);
        let mut interpreter = ImpInterpreter::new();
        assert_eq!(Value::Int(4), interpreter.visit(&*ast))
    }

    #[test]
    fn test_mod() {
        let program = "
        16 mod 2
        ";

        let ast = ImpInterpreter::parse(program);
        let mut interpreter = ImpInterpreter::new();
        assert_eq!(Value::Int(0), interpreter.visit(&*ast))
    }

    #[test]
    fn test_pow() {
        let program = "
        2^(2^3)
        ";

        let ast = ImpInterpreter::parse(program);
        let mut interpreter = ImpInterpreter::new();
        assert_eq!(Value::Int(256), interpreter.visit(&*ast))
    }

    #[test]
    fn test_precedence() {
        let program = "
         5-2*10/2
        ";

        let ast = ImpInterpreter::parse(program);
        let mut interpreter = ImpInterpreter::new();
        assert_eq!(Value::Int(-5), interpreter.visit(&*ast))
    }

    #[test]
    fn test_assign() {
        let program = "
         let a: int = 2;
        ";

        let ast = ImpInterpreter::parse(program);
        let mut interpreter = ImpInterpreter::new();
        let res = interpreter.visit(&*ast);
        assert_eq!(Some(&Value::Int(2)), interpreter.memory.get("a"));
        assert_eq!(Value::Void, res);
    }

    #[test]
    fn test_var_exp() {
        let program = "
         let a: int = 2; print(a);
        ";

        let ast = ImpInterpreter::parse(program);
        let mut interpreter = ImpInterpreter::new();
        let res = interpreter.visit(&*ast);
        assert_eq!(Some(&Value::Int(2)), interpreter.memory.get("a"));
        assert_eq!(Value::Void, res);
    }

    #[test]
    #[should_panic(expected = "Unknown variable a")]
    fn test_undeclared_var() {
        let program = "
         a
        ";

        let ast = ImpInterpreter::parse(program);
        let mut interpreter = ImpInterpreter::new();
        let res = interpreter.visit(&*ast);
        assert_eq!(None, interpreter.memory.get("a"));
        assert_eq!(Value::Void, res);
    }

    #[test]
    fn test_program() {
        let program = "
         let base: int = 5;
         let height: int = 10;
         let area: int = base * height;
         print(area);
        ";

        let ast = ImpInterpreter::parse(program);
        let mut interpreter = ImpInterpreter::new();
        let res = interpreter.visit(&*ast);
        assert_eq!(Some(&Value::Int(5)), interpreter.memory.get("base"));
        assert_eq!(Some(&Value::Int(10)), interpreter.memory.get("height"));
        assert_eq!(Some(&Value::Int(50)), interpreter.memory.get("area"));
        assert_eq!(Value::Void, res);
    }

    #[test]
    fn test_if_true() {
        let program = "
         if 3 > 2 {
           let branch: int = true;
         }
        ";

        let ast = ImpInterpreter::parse(program);
        let mut interpreter = ImpInterpreter::new();
        let res = interpreter.visit(&*ast);
        assert_eq!(Some(&Value::Bool(true)), interpreter.memory.get("branch"));
        assert_eq!(Value::Void, res);
    }

    #[test]
    fn test_if_false() {
        let program = "
         if 3 < 2 {
           let branch: int = true;
         }
        ";

        let ast = ImpInterpreter::parse(program);
        let mut interpreter = ImpInterpreter::new();
        let res = interpreter.visit(&*ast);
        assert_eq!(None, interpreter.memory.get("branch"));
        assert_eq!(Value::Void, res);
    }

    #[test]
    fn test_while_false() {
        let program = "
        let i: int = 0;
         while i > 2 {
           i = i + 1;
         }
        ";

        let ast = ImpInterpreter::parse(program);
        let mut interpreter = ImpInterpreter::new();
        let res = interpreter.visit(&*ast);
        assert_eq!(None, interpreter.memory.get("a"));
        assert_eq!(Value::Void, res);
    }

    #[test]
    fn test_while_true() {
        let program = "
        let i: int = 0;
         while i < 2 {
           i = i + 1;
         }
        ";

        let ast = ImpInterpreter::parse(program);
        let mut interpreter = ImpInterpreter::new();
        let res = interpreter.visit(&*ast);
        assert_eq!(Some(&Value::Int(2)), interpreter.memory.get("i"));
        assert_eq!(Value::Void, res);
    }

    #[test]
    fn test_if_else_true() {
        let program = "
         if 3 > 2 {
           let branch: int = true;
         } else {
           let branch: int = false;
         }
        ";

        let ast = ImpInterpreter::parse(program);
        let mut interpreter = ImpInterpreter::new();
        let res = interpreter.visit(&*ast);
        assert_eq!(Some(&Value::Bool(true)), interpreter.memory.get("branch"));
        assert_eq!(Value::Void, res);
    }

    #[test]
    fn test_if_else_false() {
        let program = "
         if 3 < 2 {
           let branch: int = true;
         } else {
           let branch: int = false;
         }
        ";

        let ast = ImpInterpreter::parse(program);
        let mut interpreter = ImpInterpreter::new();
        let res = interpreter.visit(&*ast);
        assert_eq!(Some(&Value::Bool(false)), interpreter.memory.get("branch"));
        assert_eq!(Value::Void, res);
    }

    #[test]
    fn test_to_str() {
        let program = "
         to_str(123);
        ";

        let ast = ImpInterpreter::parse(program);
        let mut interpreter = ImpInterpreter::new();
        let res = interpreter.visit(&*ast);
        assert_eq!(Value::String("123".to_string()), res);
    }

    #[test]
    fn test_str_concat() {
        let program = "
         \"Hello, \" : \"world!\"
        ";

        let ast = ImpInterpreter::parse(program);
        let mut interpreter = ImpInterpreter::new();
        let res = interpreter.visit(&*ast);
        assert_eq!(Value::String("Hello, world!".to_string()), res);
    }

    #[test]
    fn test_single_line_comment() {
        let program = "
         // This is a single line comment
         let a: int = 5; // This is another comment
        ";

        let ast = ImpInterpreter::parse(program);
        let mut interpreter = ImpInterpreter::new();
        let res = interpreter.visit(&*ast);
        assert_eq!(Some(&Value::Int(5)), interpreter.memory.get("a"));
        assert_eq!(Value::Void, res);
    }

    #[test]
    fn test_multi_line_comment() {
        let program = "
         /* This is a multi-line comment
            let b = 10;
         */
         let a: int = 5; /* Another comment */
        ";

        let ast = ImpInterpreter::parse(program);
        let mut interpreter = ImpInterpreter::new();
        let res = interpreter.visit(&*ast);
        assert_eq!(Some(&Value::Int(5)), interpreter.memory.get("a"));
        assert_eq!(None, interpreter.memory.get("b"));
        assert_eq!(Value::Void, res);
    }

    #[test]
    fn test_string_escaping() {
        let program = r#"
         let str: string = "Hello\nWorld\t!";
        "#;

        let ast = ImpInterpreter::parse(program);
        let mut interpreter = ImpInterpreter::new();
        let res = interpreter.visit(&*ast);
        assert_eq!(
            Some(&Value::String("Hello\nWorld\t!".to_string())),
            interpreter.memory.get("str")
        );
        assert_eq!(Value::Void, res);
    }

    #[test]
    fn test_char_escaping() {
        let program = r#"
         let ch: char = '\n';
        "#;

        let ast = ImpInterpreter::parse(program);
        let mut interpreter = ImpInterpreter::new();
        let res = interpreter.visit(&*ast);
        assert_eq!(Some(&Value::Char('\n')), interpreter.memory.get("ch"));
        assert_eq!(Value::Void, res);
    }
}
