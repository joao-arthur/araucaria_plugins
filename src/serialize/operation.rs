use std::collections::BTreeMap;
use serde::{Serialize, Serializer};

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum OperandValue {
    U64(u64),
    I64(i64),
    F64(f64),
    USize(usize),
    ISize(isize),
    Bool(bool),
    Str(String),
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum Operand {
    Value(OperandValue),
    FieldPath(String),
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum Operation {
    Eq(Operand),
    Ne(Operand),
    Gt(Operand),
    Ge(Operand),
    Lt(Operand),
    Le(Operand),
    Btwn(Operand, Operand),
}


pub fn to_operand_value(value: araucaria::operation::OperandValue) -> OperandValue {
    match value {
        araucaria::operation::OperandValue::U64(value) => OperandValue::U64(value),
        araucaria::operation::OperandValue::I64(value) => OperandValue::I64(value),
        araucaria::operation::OperandValue::F64(value) => OperandValue::F64(value),
        araucaria::operation::OperandValue::Bool(value) => OperandValue::Bool(value),
        araucaria::operation::OperandValue::USize(value) => OperandValue::USize(value),
        araucaria::operation::OperandValue::ISize(value) => OperandValue::ISize(value),
        araucaria::operation::OperandValue::Str(value) => OperandValue::Str(value),
    }
}

pub fn to_operand(operation: araucaria::operation::Operand) -> Operand {
    match operation {
        araucaria::operation::Operand::Value(operand_value) => Operand::Value(to_operand_value(operand_value)),
        araucaria::operation::Operand::FieldPath(path) => Operand::FieldPath(path.clone()),
    }
}

pub fn to_operation(operation: araucaria::operation::Operation) -> Operation {
    match operation {
        araucaria::operation::Operation::Eq(operand) => Operation::Eq(to_operand(operand)),
        araucaria::operation::Operation::Ne(operand) => Operation::Ne(to_operand(operand)),
        araucaria::operation::Operation::Gt(operand) => Operation::Gt(to_operand(operand)),
        araucaria::operation::Operation::Ge(operand) => Operation::Ge(to_operand(operand)),
        araucaria::operation::Operation::Lt(operand) => Operation::Lt(to_operand(operand)),
        araucaria::operation::Operation::Le(operand) => Operation::Le(to_operand(operand)),
        araucaria::operation::Operation::Btwn(operand_a, operand_b) => Operation::Btwn(to_operand(operand_a), to_operand(operand_b)),
    }
}

#[cfg(test)]
mod tests {

    use super::{to_operand_value, to_operand, to_operation, OperandValue, Operand, Operation};

    #[test]
    fn araucaria_operand_value_to_operand_value() {
        assert_eq!(to_operand_value(araucaria::operation::OperandValue::U64(12)), OperandValue::U64(12));
        assert_eq!(to_operand_value(araucaria::operation::OperandValue::I64(-34)), OperandValue::I64(-34));
        assert_eq!(to_operand_value(araucaria::operation::OperandValue::F64(-64.5)), OperandValue::F64(-64.5));
        assert_eq!(to_operand_value(araucaria::operation::OperandValue::USize(84)), OperandValue::USize(84));
        assert_eq!(to_operand_value(araucaria::operation::OperandValue::ISize(-79)), OperandValue::ISize(-79));
    }

    #[test]
    fn araucaria_operand_to_operand() {
        assert_eq!(to_operand(araucaria::operation::Operand::FieldPath("Killing Moon".into())), Operand::FieldPath("Killing Moon".into()));
    }

    #[test]
    fn araucaria_operation_to_operation() {
    }

    #[test]
    fn serialize_operand_value() {
    }

    #[test]
    fn serialize_operand() {
    }

    #[test]
    fn serialize_operation() {
    }
}
