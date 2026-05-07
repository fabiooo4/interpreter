grammar Imp;

main : prog EOF;

prog : stmt+ | exp;

stmt : decl                                                                         # declaration
     | ID ASSIGN exp SEMICOLON                                                      # mutation
     | IF exp LBRACE stmt+ RBRACE                                                   # if
     | IF exp LBRACE true_branch=stmt+ RBRACE ELSE LBRACE false_branch=stmt+ RBRACE # ifElse
     | WHILE exp LBRACE stmt* RBRACE                                                # while
     | PRINT LPAR exp RPAR SEMICOLON                                                # print
     | TOSTR LPAR exp RPAR SEMICOLON                                                # toStr
     ;

decl : DECLARATION ID ':' TYPE ASSIGN exp SEMICOLON;

// Labels begin with # and rename each node of the ParseTree
exp : SUB? INT                                  # int
    | SUB? FLOAT                                # float
    | BOOL                                      # bool
    | STRING                                    # string
    | CHAR                                      # char
    | <assoc=right> lhs=exp POW rhs=exp         # pow
    | NOT exp                                   # not
    | lhs=exp op=(MUL | DIV | MOD) rhs=exp      # mulDivMod
    | lhs=exp op=(ADD | SUB) rhs=exp            # addSub
    | lhs=exp op=(LT  | LE | GE | GT) rhs=exp   # cmp
    | lhs=exp op=(EQ | NEQ) rhs=exp             # eq
    | lhs=exp op=(AND | OR ) rhs=exp            # andOr
    | lhs=exp STRCONCAT rhs=exp                 # strConcat
    | LPAR exp RPAR                             # paren
    | SUB LPAR exp RPAR                         # neg
    | ID                                        # id
    ;

TYPE : 'int' | 'float' | 'string' | 'char' | 'bool';

INT          : NAT;
fragment NAT : '0' | POS ;
fragment POS : [1-9][0-9]* ;

FLOAT             : INT '.' DIGIT+ ;
fragment DIGIT    : '0' | POSDIGIT ;
fragment POSDIGIT : [1-9] ;

// TODO: implement non numerical types
STRING              : '"' STR* '"' ;
fragment STR        : ~["\\] | STRING_ESC ;
fragment STRING_ESC : '\\' [btnfr"'\\] ;

CHAR              : '\'' CH '\'' ;
fragment CH       : ~['\\] | CHAR_ESC ;
fragment CHAR_ESC : '\\' [btnfr'\\] ;

BOOL : 'true' | 'false';

LPAR : '(' ;
RPAR : ')' ;

LBRACE : '{' ;
RBRACE : '}' ;

ADD  : '+'   ;
SUB  : '-'   ;
MUL  : '*'   ;
DIV  : '/'   ;
MOD  : 'mod' ;
POW  : '^'   ;

EQ : '=='  ;
NEQ : '!='  ;
LE : '<='  ;
GE : '>='  ;
LT  : '<'   ;
GT  : '>'   ;
NOT : 'not' ;
AND : 'and' ;
OR  : 'or'  ;

STRCONCAT : ':'      ;
PRINT     : 'print'  ;
TOSTR     : 'to_str' ;

IF          : 'if'    ;
ELSE        : 'else'  ;
WHILE       : 'while' ;
ASSIGN      : '='     ;
DECLARATION : 'let'   ;

ID       : [a-zA-Z]+ ;
SEMICOLON : ';'      ;

// Ignores
WS           : [ \t\n\r]+    -> skip;
COMMENT      : '/*' .*? '*/' -> skip;
LINE_COMMENT : '//' ~[\r\n]* -> skip;
