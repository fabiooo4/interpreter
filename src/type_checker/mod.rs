pub mod value_type;

use std::str::FromStr;

use antlr_rust::tree::{ParseTree, ParseTreeVisitorCompat};

use crate::{
    SyntaxTree,
    memory::{HashMemory, Memory},
    parser::{
        implexer::{ADD, AND, DIV, MOD, MUL, OR, SUB},
        impparser::{
            self, CastContextAttrs, DeclContextAttrs, IdContextAttrs, IfContextAttrs,
            IfElseContextAttrs, ImpParserContextType, MainContextAttrs, MutationContextAttrs,
            NegContextAttrs, NotContextAttrs, ParenContextAttrs, WhileContextAttrs,
        },
        impvisitor::ImpVisitorCompat,
    },
    type_checker::value_type::Type,
};

#[derive(Default, Debug)]
pub struct ImpTypeChecker {
    res: Type,

    memory: HashMemory<Type>,
}

impl ImpTypeChecker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn check(&mut self, ast: &SyntaxTree) {
        self.visit(&**ast);
    }
}

impl ParseTreeVisitorCompat<'_> for ImpTypeChecker {
    type Node = ImpParserContextType;
    type Return = Type;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.res
    }

    fn visit_error_node(
        &mut self,
        _node: &antlr_rust::tree::ErrorNode<'_, Self::Node>,
    ) -> Self::Return {
        panic!("Error encountered: {}", _node.symbol)
    }

    fn visit(
        &mut self,
        node: &<Self::Node as antlr_rust::parser::ParserNodeType<'_>>::Type,
    ) -> Self::Return {
        node.accept(self);

        // Set the final result in the type checker
        self.res
    }
}

impl ImpVisitorCompat<'_> for ImpTypeChecker {
    fn visit_main(&mut self, ctx: &impparser::MainContext<'_>) -> Self::Return {
        self.visit(&*ctx.prog().unwrap())
    }

    //
    // Types {
    //

    fn visit_int(&mut self, _ctx: &crate::parser::impparser::IntContext<'_>) -> Self::Return {
        Type::Int
    }

    fn visit_float(&mut self, _ctx: &crate::parser::impparser::FloatContext<'_>) -> Self::Return {
        Type::Float
    }

    fn visit_bool(&mut self, _ctx: &crate::parser::impparser::BoolContext<'_>) -> Self::Return {
        Type::Bool
    }

    fn visit_string(&mut self, _ctx: &crate::parser::impparser::StringContext<'_>) -> Self::Return {
        Type::String
    }

    fn visit_char(&mut self, _ctx: &crate::parser::impparser::CharContext<'_>) -> Self::Return {
        Type::Char
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
        let lhs = self.visit(&*ctx.lhs.clone().unwrap());
        let rhs = self.visit(&*ctx.rhs.clone().unwrap());

        lhs.cmp_type(rhs)
    }

    fn visit_eq(&mut self, ctx: &impparser::EqContext<'_>) -> Self::Return {
        let lhs = self.visit(&*ctx.lhs.clone().unwrap());
        let rhs = self.visit(&*ctx.rhs.clone().unwrap());

        if lhs == rhs {
            Type::Bool
        } else {
            panic!("Type mismatch: cannot compare '{}' and '{}'", lhs, rhs)
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
        let exp_typ = self.visit(&*ctx.exp().unwrap());
        let cast_typ = &*ctx.TYPE().unwrap().get_text();

        let cast_typ = Type::from_str(&ctx.TYPE().unwrap().get_text())
            .unwrap_or_else(|e| panic!("Failed to convert '{exp_typ}' to '{cast_typ}': {e}"));

        exp_typ.cast(cast_typ)
    }

    fn visit_paren(&mut self, ctx: &impparser::ParenContext<'_>) -> Self::Return {
        self.visit(&*ctx.exp().unwrap())
    }

    fn visit_toStr(&mut self, _ctx: &impparser::ToStrContext<'_>) -> Self::Return {
        // TODO: Check that the value can be converted to string

        Type::String
    }

    fn visit_strConcat(&mut self, ctx: &impparser::StrConcatContext<'_>) -> Self::Return {
        let lhs = self.visit(&*ctx.lhs.clone().unwrap());
        let rhs = self.visit(&*ctx.rhs.clone().unwrap());

        lhs.concat(rhs)
    }

    // } Expressions

    //
    // Variables {
    //

    fn visit_decl(&mut self, ctx: &impparser::DeclContext<'_>) -> Self::Return {
        let id = ctx.ID().unwrap().get_text();
        let val_typ = self.visit(&*ctx.exp().unwrap());

        let id_typ = ctx
            .TYPE()
            .unwrap()
            .get_text()
            .parse()
            .unwrap_or_else(|e| panic!("{e}"));

        if id_typ == val_typ {
            self.memory.add(id, id_typ);
        } else {
            panic!("Mismatched types: expected '{}' got '{}'", id_typ, val_typ);
        }

        Type::Void
    }

    fn visit_id(&mut self, ctx: &impparser::IdContext<'_>) -> Self::Return {
        let id = ctx.ID().unwrap().get_text();
        *self
            .memory
            .get(&id)
            .unwrap_or_else(|| panic!("Unknown variable {}", id))
    }

    fn visit_mutation(&mut self, ctx: &impparser::MutationContext<'_>) -> Self::Return {
        let id = ctx.ID().unwrap().get_text();
        let val_type = self.visit(&*ctx.exp().unwrap());
        let id_type = self
            .memory
            .get(&id)
            .unwrap_or_else(|| panic!("Unknown variable {}", id));

        if *id_type == val_type {
            self.memory
                .update(id.clone(), val_type)
                .unwrap_or_else(|| panic!("Unknown variable {}", id));
        } else {
            panic!(
                "Mismatched types: expected '{}' got '{}'",
                id_type, val_type
            );
        }

        Type::Void
    }
    // } Variables

    //
    // Statements {
    //

    fn visit_print(&mut self, _ctx: &impparser::PrintContext<'_>) -> Self::Return {
        // TODO: check if the value is convertable to string

        Type::Void
    }

    fn visit_if(&mut self, ctx: &impparser::IfContext<'_>) -> Self::Return {
        let condition = self.visit(&*ctx.exp().unwrap());
        if let Type::Bool = condition {
            Type::Void
        } else {
            panic!(
                "Mismatched types: expected '{}' got '{}'",
                Type::Bool,
                condition
            );
        }
    }

    fn visit_ifElse(&mut self, ctx: &impparser::IfElseContext<'_>) -> Self::Return {
        let condition = self.visit(&*ctx.exp().unwrap());
        if let Type::Bool = condition {
            Type::Void
        } else {
            panic!(
                "Mismatched types: expected '{}' got '{}'",
                Type::Bool,
                condition
            );
        }
    }

    fn visit_while(&mut self, ctx: &impparser::WhileContext<'_>) -> Self::Return {
        let condition = self.visit(&*ctx.exp().unwrap());
        if let Type::Bool = condition {
            Type::Void
        } else {
            panic!(
                "Mismatched types: expected '{}' got '{}'",
                Type::Bool,
                condition
            );
        }
    }

    // } Statements
}
