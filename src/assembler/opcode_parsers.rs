use nom::do_parse;
use nom::types::CompleteStr;
use nom::*;

use crate::assembler::Token;
use crate::instruction::Opcode;

named!(pub opcode_load<CompleteStr, Token>,
  do_parse!(
      tag!("load") >> (Token::Op{code: Opcode::LOAD})
  )
);

named!(pub integer_operand<CompleteStr, Token>,
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

mod tests {
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
