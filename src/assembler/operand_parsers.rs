use crate::assembler::label_parsers::label_usage;
use crate::assembler::register_parsers::register;
use nom::named;
use nom::types::CompleteStr;
use nom::*;

use crate::assembler::Token;

named!(integer_operand<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("#") >>
            reg_num: digit >>
            (
                Token::IntegerOperand{value: reg_num.parse::<i32>().unwrap()}
            )
        )
    )
);

named!(pub operand<CompleteStr, Token>,
    alt!(
        integer_operand |
        label_usage |
        register |
        irstring
    )
);

named!(irstring<CompleteStr, Token>,
    do_parse!(
        tag!("'") >>
        content: take_until!("'") >>
        tag!("'") >>
        (
            Token::IrString{ name: content.to_string() }
        )
    )
);

mod tests {
    #![allow(unused_imports)]
    use super::integer_operand;
    use crate::assembler::Token;
    use nom::types::CompleteStr;

    #[test]
    fn test_parse_integer_operand() {
        let result = integer_operand(CompleteStr("#10"));
        assert_eq!(result.is_ok(), true);
        let (rest, value) = result.unwrap();
        assert_eq!(rest, CompleteStr(""));
        assert_eq!(value, Token::IntegerOperand { value: 10 });

        let result = integer_operand(CompleteStr("10"));
        assert_eq!(result.is_ok(), false);
    }
}
