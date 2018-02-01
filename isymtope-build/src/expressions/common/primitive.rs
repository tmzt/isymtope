use error::*;
use traits::*;
use expressions::*;
use output::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Primitive {
    CharVal(char),
    StringVal(String),
    Int32Val(i32),
    BoolVal(bool),
    NullVal,
    Undefined,
}

impl TryProcessFrom<ExpressionValue<OutputExpression>> for Primitive {
    fn try_process_from(
        src: &ExpressionValue<OutputExpression>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        match *src {
            ExpressionValue::Primitive(ref p) => Ok(p.to_owned()),
            _ => Err(try_process_from_err!("Cannot evaluate as primitive.")),
        }
    }
}

impl TryEvalFrom<ExpressionValue<OutputExpression>> for Primitive {
    fn try_eval_from(
        src: &ExpressionValue<OutputExpression>,
        ctx: &mut OutputContext<ProcessedExpression>,
    ) -> DocumentProcessingResult<Self> {
        match *src {
            ExpressionValue::Primitive(ref p) => Ok(p.to_owned()),
            _ => Err(try_process_from_err!("Cannot evaluate as primitive.")),
        }
    }
}

impl TryProcessFrom<ExpressionValue<OutputExpression>> for bool {
    fn try_process_from(
        src: &ExpressionValue<OutputExpression>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        match *src {
            ExpressionValue::Primitive(Primitive::BoolVal(b)) if b => Ok(true),
            ExpressionValue::Primitive(Primitive::BoolVal(_)) => Ok(false),
            _ => Err(try_process_from_err!("Cannot evaluate as bool.")),
        }
    }
}

impl TryEvalFrom<ExpressionValue<OutputExpression>> for bool {
    fn try_eval_from(
        src: &ExpressionValue<OutputExpression>,
        ctx: &mut OutputContext<ProcessedExpression>,
    ) -> DocumentProcessingResult<Self> {
        match *src {
            ExpressionValue::Primitive(Primitive::BoolVal(b)) if b => Ok(true),
            ExpressionValue::Primitive(Primitive::BoolVal(_)) => Ok(false),
            _ => Err(try_process_from_err!("Cannot evaluate as bool.")),
        }
    }
}
