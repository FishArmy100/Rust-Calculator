# Grammer:
### Tokens:
- IDENTIFIER: any sequence of alpha-numeric charcactors, that starts with a letter or underscore
    - `foo`
    - `x1`
    - `_y`
- LITERAL: A sequence of digits, with an optional `'.'`, and a second sequence of digits
    - `590`
    - `0.456`
    - `4589.33333`
- Mathmatical Tokens:
    - PLUS: `+`
    - MINUS: `-`
    - STAR: `*`
    - SLASH: `/`
    - CARRET: `^`
- Miscilanius:
    - OPEN_PAREN: `(`
    - CLOSE_PAREN: `)`
    - SEMI_COLON: `;`
    - EQUALS: `=`
    - COMMA: `,`

### Parsing:
- Grouping = `OPEN_PAREN` Expression `CLOSE_PAREN`;
- Call = `IDENTIFIER` `OPEN_PAREN` Expression (`COMMA` Expression)* `CLOSE_PAREN`;

- Primary = `IDENTIFIER` | `LITERAL` | Grouping | Call;
- Exponent = Primary `CARRET` (Exponent | Primary);
- Unary = MINUS (Unary | Exponent);
- Factor = Unary ((`STAR` | `SLASH`) Unary)*;
- Term = Factor ((`PLUS` | `MINUS`) Factor)*;
- Expression = Term;

- Variable = `IDENTIFIER` `EQUALS` Expression `SEMI_COLON`;

- Function = `IDENTIFIER` `OPEN_PAREN` `IDENTIFIER` (`COMMA` `IDENTIFIER`)* `CLOSE_PAREN` `EQUALS` Expression `SEMI_COLON`;

- Statement = Variable | Function

- Program = Statement+ | Expression
