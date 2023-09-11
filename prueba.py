x = (
    'WHILE(Binary(Variable("x"), Token { token_type: LESS, lexeme: \'<\', line: 1 }, NumberLit(10.0)), BLOCK([PRINT(Variable("x")), BLOCK([EXPR(Assignment("x", Binary(Variable("x"), Token { token_type: PLUS, lexeme: '
    + ', line: 4 }, NumberLit(1.0))))])]))\
Some(Token { token_type: IDENTIFIER("x"), lexeme: "x", line: 8 })'
)

y = (
    'WHILE(Binary(Variable("x"), Token { token_type: LESS, lexeme: "<", line: 8 }, NumberLit(10.0)), BLOCK([BLOCK([PRINT(Variable("x"))]), EXPR(Assignment("x", Binary(Variable("x"), Token { token_type: PLUS, lexeme: '
    + ", line: 8 }, NumberLit(1.0))))]))"
)
