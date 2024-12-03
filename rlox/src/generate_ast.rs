use paste::paste;

use crate::token::{Object, Token};

macro_rules! generate_ast {
    ($name:ident, [$( $varient:ident : {$($field:ident: $type:ty),*}),*]) => {
        paste!{enum $name {
            $($varient([<$varient $name>]),)*
        }}


        paste!{
        $(struct [<$varient $name>] {
            $($field: $type,)*
        })*
        }

        paste!{
        $(impl [<$varient $name>] {
            pub fn new($($field: $type),*) -> Self {
                Self {
                    $($field),*
                }
            }
        })*
        }
    };
}

generate_ast!(Expr,
[Binary : {left: Box<Expr>, operator: Token, right: Box<Expr>},
 Grouping : {expression: Box<Expr>},
 Literal : {value: Object},
 Unary : {operator: Token, right: Box<Expr>}
]);
