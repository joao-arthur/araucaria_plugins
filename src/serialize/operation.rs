use serde::{Serialize, Serializer};

#[derive(Debug, PartialEq, Clone)]
pub enum OperandValue {
    U64(u64),
    I64(i64),
    F64(f64),
    USize(usize),
    ISize(isize),
    Bool(bool),
    Str(String),
}

impl Serialize for OperandValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            OperandValue::U64(value) => value.serialize(serializer),
            OperandValue::I64(value) => value.serialize(serializer),
            OperandValue::F64(value) => value.serialize(serializer),
            OperandValue::USize(value) => value.serialize(serializer),
            OperandValue::ISize(value) => value.serialize(serializer),
            OperandValue::Bool(value) => value.serialize(serializer),
            OperandValue::Str(value) => value.serialize(serializer),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operand {
    Value(OperandValue),
    FieldPath(String),
}

impl Serialize for Operand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Operand::Value(value) => value.serialize(serializer),
            Operand::FieldPath(value) => value.serialize(serializer),
        }
    }
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

pub fn to_operand_value(operand_value: araucaria::operation::OperandValue) -> OperandValue {
    match operand_value {
        araucaria::operation::OperandValue::U64(operand_value) => OperandValue::U64(operand_value),
        araucaria::operation::OperandValue::I64(operand_value) => OperandValue::I64(operand_value),
        araucaria::operation::OperandValue::F64(operand_value) => OperandValue::F64(operand_value),
        araucaria::operation::OperandValue::Bool(operand_value) => OperandValue::Bool(operand_value),
        araucaria::operation::OperandValue::USize(operand_value) => OperandValue::USize(operand_value),
        araucaria::operation::OperandValue::ISize(operand_value) => OperandValue::ISize(operand_value),
        araucaria::operation::OperandValue::Str(operand_value) => OperandValue::Str(operand_value),
    }
}

pub fn to_operand(operand: araucaria::operation::Operand) -> Operand {
    match operand {
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
    use std::sync::LazyLock;

    use super::{Operand, OperandValue, Operation, to_operand, to_operand_value, to_operation};

    const VALUE_U64: u64 = 12;
    const VALUE_I64: i64 = -34;
    const VALUE_F64: f64 = -64.5;
    const VALUE_USIZE: usize = 84;
    const VALUE_ISIZE: isize = -79;
    const VALUE_BOOL: bool = false;
    const VALUE_STR: &str = "O sol vê tudo, mas não conhece o amor";
    const FIELD: &str = "info.data.details.value";

    const ARAUCARIA_OPERAND_VALUE_U64: araucaria::operation::OperandValue = araucaria::operation::OperandValue::U64(VALUE_U64);
    const ARAUCARIA_OPERAND_VALUE_I64: araucaria::operation::OperandValue = araucaria::operation::OperandValue::I64(VALUE_I64);
    const ARAUCARIA_OPERAND_VALUE_F64: araucaria::operation::OperandValue = araucaria::operation::OperandValue::F64(VALUE_F64);
    const ARAUCARIA_OPERAND_VALUE_USIZE: araucaria::operation::OperandValue = araucaria::operation::OperandValue::USize(VALUE_USIZE);
    const ARAUCARIA_OPERAND_VALUE_ISIZE: araucaria::operation::OperandValue = araucaria::operation::OperandValue::ISize(VALUE_ISIZE);
    const ARAUCARIA_OPERAND_VALUE_BOOL: araucaria::operation::OperandValue = araucaria::operation::OperandValue::Bool(VALUE_BOOL);
    static ARAUCARIA_OPERAND_VALUE_STR: LazyLock<araucaria::operation::OperandValue> =
        LazyLock::new(|| araucaria::operation::OperandValue::Str(VALUE_STR.into()));

    const ARAUCARIA_OPERAND_U64: araucaria::operation::Operand = araucaria::operation::Operand::Value(ARAUCARIA_OPERAND_VALUE_U64);
    const ARAUCARIA_OPERAND_I64: araucaria::operation::Operand = araucaria::operation::Operand::Value(ARAUCARIA_OPERAND_VALUE_I64);
    const ARAUCARIA_OPERAND_F64: araucaria::operation::Operand = araucaria::operation::Operand::Value(ARAUCARIA_OPERAND_VALUE_F64);
    const ARAUCARIA_OPERAND_USIZE: araucaria::operation::Operand = araucaria::operation::Operand::Value(ARAUCARIA_OPERAND_VALUE_USIZE);
    const ARAUCARIA_OPERAND_ISIZE: araucaria::operation::Operand = araucaria::operation::Operand::Value(ARAUCARIA_OPERAND_VALUE_ISIZE);
    const ARAUCARIA_OPERAND_BOOL: araucaria::operation::Operand = araucaria::operation::Operand::Value(ARAUCARIA_OPERAND_VALUE_BOOL);
    static ARAUCARIA_OPERAND_STR: LazyLock<araucaria::operation::Operand> =
        LazyLock::new(|| araucaria::operation::Operand::Value(ARAUCARIA_OPERAND_VALUE_STR.clone()));
    static ARAUCARIA_OPERAND_FIELD: LazyLock<araucaria::operation::Operand> =
        LazyLock::new(|| araucaria::operation::Operand::FieldPath(FIELD.into()));

    const ARAUCARIA_OPERATION_EQ: araucaria::operation::Operation = araucaria::operation::Operation::Eq(ARAUCARIA_OPERAND_U64);
    const ARAUCARIA_OPERATION_NE: araucaria::operation::Operation = araucaria::operation::Operation::Ne(ARAUCARIA_OPERAND_I64);
    const ARAUCARIA_OPERATION_GT: araucaria::operation::Operation = araucaria::operation::Operation::Gt(ARAUCARIA_OPERAND_F64);
    const ARAUCARIA_OPERATION_GE: araucaria::operation::Operation = araucaria::operation::Operation::Ge(ARAUCARIA_OPERAND_USIZE);
    const ARAUCARIA_OPERATION_LT: araucaria::operation::Operation = araucaria::operation::Operation::Lt(ARAUCARIA_OPERAND_ISIZE);
    const ARAUCARIA_OPERATION_LE: araucaria::operation::Operation = araucaria::operation::Operation::Le(ARAUCARIA_OPERAND_BOOL);
    const ARAUCARIA_OPERATION_BTWN: LazyLock<araucaria::operation::Operation> =
        LazyLock::new(|| araucaria::operation::Operation::Btwn(ARAUCARIA_OPERAND_STR.clone(), ARAUCARIA_OPERAND_FIELD.clone()));

    const OPERAND_VALUE_U64: OperandValue = OperandValue::U64(VALUE_U64);
    const OPERAND_VALUE_I64: OperandValue = OperandValue::I64(VALUE_I64);
    const OPERAND_VALUE_F64: OperandValue = OperandValue::F64(VALUE_F64);
    const OPERAND_VALUE_USIZE: OperandValue = OperandValue::USize(VALUE_USIZE);
    const OPERAND_VALUE_ISIZE: OperandValue = OperandValue::ISize(VALUE_ISIZE);
    const OPERAND_VALUE_BOOL: OperandValue = OperandValue::Bool(VALUE_BOOL);
    static OPERAND_VALUE_STR: LazyLock<OperandValue> = LazyLock::new(|| OperandValue::Str(VALUE_STR.into()));

    const OPERAND_U64: Operand = Operand::Value(OPERAND_VALUE_U64);
    const OPERAND_I64: Operand = Operand::Value(OPERAND_VALUE_I64);
    const OPERAND_F64: Operand = Operand::Value(OPERAND_VALUE_F64);
    const OPERAND_USIZE: Operand = Operand::Value(OPERAND_VALUE_USIZE);
    const OPERAND_ISIZE: Operand = Operand::Value(OPERAND_VALUE_ISIZE);
    const OPERAND_BOOL: Operand = Operand::Value(OPERAND_VALUE_BOOL);
    const OPERAND_STR: LazyLock<Operand> = LazyLock::new(|| Operand::Value(OPERAND_VALUE_STR.clone()));
    static OPERAND_FIELD: LazyLock<Operand> = LazyLock::new(|| Operand::FieldPath(FIELD.into()));

    const OPERATION_EQ: Operation = Operation::Eq(OPERAND_U64);
    const OPERATION_NE: Operation = Operation::Ne(OPERAND_I64);
    const OPERATION_GT: Operation = Operation::Gt(OPERAND_F64);
    const OPERATION_GE: Operation = Operation::Ge(OPERAND_USIZE);
    const OPERATION_LT: Operation = Operation::Lt(OPERAND_ISIZE);
    const OPERATION_LE: Operation = Operation::Le(OPERAND_BOOL);
    const OPERATION_BTWN: LazyLock<Operation> = LazyLock::new(|| Operation::Btwn(OPERAND_STR.clone(), OPERAND_FIELD.clone()));

    #[test]
    fn araucaria_operand_value_to_operand_value() {
        assert_eq!(to_operand_value(ARAUCARIA_OPERAND_VALUE_U64), OPERAND_VALUE_U64);
        assert_eq!(to_operand_value(ARAUCARIA_OPERAND_VALUE_I64), OPERAND_VALUE_I64);
        assert_eq!(to_operand_value(ARAUCARIA_OPERAND_VALUE_F64), OPERAND_VALUE_F64);
        assert_eq!(to_operand_value(ARAUCARIA_OPERAND_VALUE_USIZE), OPERAND_VALUE_USIZE);
        assert_eq!(to_operand_value(ARAUCARIA_OPERAND_VALUE_ISIZE), OPERAND_VALUE_ISIZE);
        assert_eq!(to_operand_value(ARAUCARIA_OPERAND_VALUE_BOOL), OPERAND_VALUE_BOOL);
        assert_eq!(to_operand_value(ARAUCARIA_OPERAND_VALUE_STR.clone()), OPERAND_VALUE_STR.clone());
    }

    #[test]
    fn araucaria_operand_to_operand() {
        assert_eq!(to_operand(ARAUCARIA_OPERAND_U64), OPERAND_U64);
        assert_eq!(to_operand(ARAUCARIA_OPERAND_I64), OPERAND_I64);
        assert_eq!(to_operand(ARAUCARIA_OPERAND_F64), OPERAND_F64);
        assert_eq!(to_operand(ARAUCARIA_OPERAND_USIZE), OPERAND_USIZE);
        assert_eq!(to_operand(ARAUCARIA_OPERAND_ISIZE), OPERAND_ISIZE);
        assert_eq!(to_operand(ARAUCARIA_OPERAND_BOOL), OPERAND_BOOL);
        assert_eq!(to_operand(ARAUCARIA_OPERAND_STR.clone()), OPERAND_STR.clone());
        assert_eq!(to_operand(ARAUCARIA_OPERAND_FIELD.clone()), OPERAND_FIELD.clone());
    }

    #[test]
    fn araucaria_operation_to_operation() {
        assert_eq!(to_operation(ARAUCARIA_OPERATION_EQ), OPERATION_EQ);
        assert_eq!(to_operation(ARAUCARIA_OPERATION_NE), OPERATION_NE);
        assert_eq!(to_operation(ARAUCARIA_OPERATION_GT), OPERATION_GT);
        assert_eq!(to_operation(ARAUCARIA_OPERATION_GE), OPERATION_GE);
        assert_eq!(to_operation(ARAUCARIA_OPERATION_LT), OPERATION_LT);
        assert_eq!(to_operation(ARAUCARIA_OPERATION_LE), OPERATION_LE);
        assert_eq!(to_operation(ARAUCARIA_OPERATION_BTWN.clone()), OPERATION_BTWN.clone());
    }

    #[test]
    fn serialize_operand_value() {
        assert_eq!(serde_json::to_string(&OPERAND_VALUE_U64).unwrap(), "12".to_string());
        assert_eq!(serde_json::to_string(&OPERAND_VALUE_I64).unwrap(), "-34".to_string());
        assert_eq!(serde_json::to_string(&OPERAND_VALUE_F64).unwrap(), "-64.5".to_string());
        assert_eq!(serde_json::to_string(&OPERAND_VALUE_USIZE).unwrap(), "84".to_string());
        assert_eq!(serde_json::to_string(&OPERAND_VALUE_ISIZE).unwrap(), "-79".to_string());
        assert_eq!(serde_json::to_string(&OPERAND_VALUE_BOOL).unwrap(), "false".to_string());
        assert_eq!(serde_json::to_string(&OPERAND_VALUE_STR.clone()).unwrap(), r#""O sol vê tudo, mas não conhece o amor""#.to_string());
    }

    #[test]
    fn serialize_operand() {
        assert_eq!(serde_json::to_string(&OPERAND_U64).unwrap(), "12".to_string());
        assert_eq!(serde_json::to_string(&OPERAND_I64).unwrap(), "-34".to_string());
        assert_eq!(serde_json::to_string(&OPERAND_F64).unwrap(), "-64.5".to_string());
        assert_eq!(serde_json::to_string(&OPERAND_USIZE).unwrap(), "84".to_string());
        assert_eq!(serde_json::to_string(&OPERAND_ISIZE).unwrap(), "-79".to_string());
        assert_eq!(serde_json::to_string(&OPERAND_BOOL).unwrap(), "false".to_string());
        assert_eq!(serde_json::to_string(&OPERAND_STR.clone()).unwrap(), r#""O sol vê tudo, mas não conhece o amor""#.to_string());
        assert_eq!(serde_json::to_string(&OPERAND_FIELD.clone()).unwrap(), r#""info.data.details.value""#.to_string());
    }

    #[test]
    fn serialize_operation() {
        assert_eq!(serde_json::to_string(&OPERATION_EQ).unwrap(), r#"{"Eq":12}"#.to_string());
        assert_eq!(serde_json::to_string(&OPERATION_NE).unwrap(), r#"{"Ne":-34}"#.to_string());
        assert_eq!(serde_json::to_string(&OPERATION_GT).unwrap(), r#"{"Gt":-64.5}"#.to_string());
        assert_eq!(serde_json::to_string(&OPERATION_GE).unwrap(), r#"{"Ge":84}"#.to_string());
        assert_eq!(serde_json::to_string(&OPERATION_LT).unwrap(), r#"{"Lt":-79}"#.to_string());
        assert_eq!(serde_json::to_string(&OPERATION_LE).unwrap(), r#"{"Le":false}"#.to_string());
        assert_eq!(
            serde_json::to_string(&OPERATION_BTWN.clone()).unwrap(),
            r#"{"Btwn":["O sol vê tudo, mas não conhece o amor","info.data.details.value"]}"#.to_string()
        );
    }
}
