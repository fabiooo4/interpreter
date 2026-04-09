grammar IntExpr;
import Nat;

main : exp EOF;

// Labels begin with # and rename each node of the ParseTree
exp : nat                                  # val
    | LPAR left=exp ADD right=exp RPAR     # add
    | LPAR left=exp MUL right=exp RPAR     # mul
    ;

LPAR : '(' ;
RPAR : ')' ;
ADD  : '+' ;
MUL  : '*' ;

// This rule ignores all whitespace
WS   : [ \t\n\r]+ -> skip ;
