use core::panic;

use antlr_rust::tree::{ErrorNode, ParseTree, ParseTreeVisitorCompat};

use crate::parser::{
    intexprparser::{
        AddContext, IntExprParserContextType, MainContext, MainContextAttrs, ValContext,
    },
    intexprvisitor::IntExprVisitorCompat,
};

pub struct IntExprInterpreter(pub u32);

impl ParseTreeVisitorCompat<'_> for IntExprInterpreter {
    type Node = IntExprParserContextType;
    type Return = u32;

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

impl IntExprVisitorCompat<'_> for IntExprInterpreter {
    fn visit_main(&mut self, ctx: &MainContext<'_>) -> Self::Return {
        self.visit(&*ctx.exp().unwrap())
    }

    fn visit_val(&mut self, ctx: &ValContext<'_>) -> Self::Return {
        ctx.get_text()
            .parse()
            .expect("Failed to parse integer value")
    }

    fn visit_add(&mut self, ctx: &AddContext<'_>) -> Self::Return {
        let left = self.visit(
            &*ctx
                .left
                .clone()
                .expect("Failed to parse left value for addition"),
        );
        let right = self.visit(
            &*ctx
                .right
                .clone()
                .expect("Failed to parse right value for addition"),
        );

        left + right
    }

    fn visit_mul(&mut self, ctx: &crate::parser::intexprparser::MulContext<'_>) -> Self::Return {
        let left = self.visit(
            &*ctx
                .left
                .clone()
                .expect("Failed to parse left value for multiplication"),
        );
        let right = self.visit(
            &*ctx
                .right
                .clone()
                .expect("Failed to parse right value for multiplication"),
        );

        left * right
    }
}
