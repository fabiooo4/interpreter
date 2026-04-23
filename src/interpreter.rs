use core::panic;

use antlr_rust::{
    token::Token,
    tree::{ErrorNode, ParseTree, ParseTreeVisitorCompat},
};

use crate::parser::{
    impparser::{
        self, ADD, DIV, ImpParserContextType, MOD, MUL, MainContext, MainContextAttrs, NegContext,
        PowContext, Prec1opContext, Prec2opContext, SUB, ValContext,
    },
    impvisitor::ImpVisitorCompat,
};

pub struct ImpInterpreter(pub f32);

impl ParseTreeVisitorCompat<'_> for ImpInterpreter {
    type Node = ImpParserContextType;
    type Return = f32;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.0
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
                .expect("Failed to parse left value for addition"),
        );
        let rhs = self.visit(
            &*ctx
                .rhs
                .clone()
                .expect("Failed to parse right value for addition"),
        );

        match operator {
            MUL => lhs * rhs,
            DIV => lhs / rhs,
            MOD => lhs % rhs,
            _ => {
                todo!("error")
            }
        }
    }

    fn visit_pow(&mut self, ctx: &PowContext<'_>) -> Self::Return {
        let lhs = self.visit(
            &*ctx
                .lhs
                .clone()
                .expect("Failed to parse left value for addition"),
        );
        let rhs = self.visit(
            &*ctx
                .rhs
                .clone()
                .expect("Failed to parse right value for addition"),
        );

        lhs.powf(rhs)
    }

    fn visit_prec2op(&mut self, ctx: &Prec2opContext<'_>) -> Self::Return {
        let operator = ctx.op.to_owned().unwrap().token_type;

        let lhs = self.visit(
            &*ctx
                .lhs
                .clone()
                .expect("Failed to parse left value for addition"),
        );
        let rhs = self.visit(
            &*ctx
                .rhs
                .clone()
                .expect("Failed to parse right value for addition"),
        );

        match operator {
            ADD => lhs + rhs,
            SUB => lhs - rhs,
            _ => {
                todo!("error")
            }
        }
    }

    fn visit_neg(&mut self, ctx: &NegContext<'_>) -> Self::Return {
        todo!("visit_neg")
    }

    //
    // Variables
    //

    fn visit_declaration(&mut self, ctx: &impparser::DeclarationContext<'_>) -> Self::Return {
        todo!("visit_declaration")
    }
}
