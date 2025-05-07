use crate::token::{Literal, Token};

// 定义AST的宏（支持你期望的语法）
macro_rules! define_ast {
    (
        $(
            ($node:ident ( $($param:ident : $type:ty ),* ), $visitor:ident)
        ),+
    ) => {
        // AST节点枚举定义
        #[derive(Debug)]
        pub enum Expression {
            $(
                $node {
                    $($param: $type),*
                },
            )+
        }

        // Visitor trait定义
        pub trait ExprVisitor<T> {
            $(
                fn $visitor(&self, $($param: &$type),*) -> T;
            )+
        }

        // 实现accept方法
        impl Expression {
            pub fn accept<V: ExprVisitor<T>, T>(&self, visitor: &V) -> T {
                match self {
                    $(
                        Expression::$node { $($param),* } => {
                            visitor.$visitor($($param),*)
                        }
                    ),+
                }
            }
        }
    };
}

define_ast! {
    (Binary(left: Box<Expression>, operator: Token, right: Box<Expression>), visit_binary),
    (Literal(value: Literal), visit_literal),
    (Grouping(expr: Box<Expression>), visit_grouping),
    (Unary(operator: Token, right: Box<Expression>), visit_unary)
}

struct AstPrinter;

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary(&self, left: &Box<Expression>, op: &Token, right: &Box<Expression>) -> String {
        format!(
            "({} {} {})",
            op.lexeme(),
            left.accept(self),
            right.accept(self)
        )
    }

    fn visit_literal(&self, value: &Literal) -> String {
        Literal::to_string(value)
    }

    fn visit_grouping(&self, expr: &Box<Expression>) -> String {
        format!("(group {})", expr.accept(self))
    }

    fn visit_unary(&self, operator: &Token, right: &Box<Expression>) -> String {
        format!("({} {})", operator.lexeme(), right.accept(self))
    }
}

// 测试代码
#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenType;

    #[test]
    fn test_ast() {
        let expr = Expression::Binary {
            left: Box::new(Expression::Literal {
                value: Literal::String("1".to_string()),
            }),
            operator: Token::new(TokenType::Plus, "+", Literal::Null, 1),
            right: Box::new(Expression::Grouping {
                expr: Box::new(Expression::Literal {
                    value: Literal::String("2".to_string()),
                }),
            }),
        };

        let printer = AstPrinter;
        assert_eq!(expr.accept(&printer), "(+ 1 (group 2))");
    }
}
