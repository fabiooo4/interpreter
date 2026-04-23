grammar Imp;

main : prog EOF;

prog: decl exp;

// Labels begin with # and rename each node of the ParseTree
exp : SUB? typ=(INT | FLOAT)             # val
    | lhs=exp op=(MUL | DIV | MOD) rhs=exp  # prec1op
    | <assoc=right> lhs=exp POW rhs=exp  # pow
    | lhs=exp op=(ADD | SUB) rhs=exp        # prec2op
    | LPAR exp RPAR                      # paren
    | SUB LPAR exp RPAR                  # neg
    | VAR                                # var
    ;

decl: VAR DECL exp SEMICOLON decl # declaration
    |                             # nildeclaration
    ;

INT          : NAT;
fragment NAT : '0' | POS ;
fragment POS : [1-9][0-9]* ;

FLOAT             : INT '.' DIGIT+ ;
fragment DIGIT    : '0' | POSDIGIT ;
fragment POSDIGIT : [1-9] ;

// TODO: implement non numerical types
STRING : '"' STR* '"' ;
fragment STR : ~["\\] | STRING_ESC ;
fragment STRING_ESC : '\\' [btnfr"'\\] ;

CHAR : '\'' CH* '\'' ;
fragment CH : ~['\\] | CHAR_ESC ;
fragment CHAR_ESC : '\\' [btnfr'\\] ;

LPAR : '(';
RPAR : ')';

ADD  : '+';
SUB  : '-';
MUL  : '*';
DIV  : '/';
MOD  : 'mod';
POW  : '^';

VAR       : [a-zA-Z]+;
DECL      : '=';
SEMICOLON : ';';

// Ignores
WS           : [ \t\n\r]+    -> skip;
COMMENT      : '/*' .*? '*/' -> skip;
LINE_COMMENT : '//' ~[\r\n]* -> skip;
