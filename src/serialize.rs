use std::collections::BTreeMap;

use serde::{Serialize, Serializer};

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum Value {
    None,
    U64(u64),
    I64(i64),
    F64(f64),
    USize(usize),
    ISize(isize),
    Bool(bool),
    Str(String),
    Arr(Vec<Value>),
    Obj(BTreeMap<String, Value>),
}

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

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum ValidationErr {
    Required,
    U64,
    I64,
    F64,
    USize,
    ISize,
    Bool,
    Str,
    Email,
    Date,
    Time,
    DateTime,
    Operation(Operation),
    BytesLen(Operation),
    CharsLen(Operation),
    GraphemesLen(Operation),
    LowercaseLen(Operation),
    UppercaseLen(Operation),
    NumbersLen(Operation),
    SymbolsLen(Operation),
    USizeEnum(Vec<usize>),
    ISizeEnum(Vec<isize>),
    StrEnum(Vec<String>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum SchemaErr {
    Arr(Vec<ValidationErr>),
    Obj(BTreeMap<String, SchemaErr>),
}

impl Serialize for SchemaErr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SchemaErr::Arr(vec) => vec.serialize(serializer),
            SchemaErr::Obj(map) => map.serialize(serializer),
        }
    }
}

pub fn map_value(value: araucaria::value::Value) -> Value {
    match value {
        araucaria::value::Value::None => Value::None,
        araucaria::value::Value::U64(value) => Value::U64(value),
        araucaria::value::Value::I64(value) => Value::I64(value),
        araucaria::value::Value::F64(value) => Value::F64(value),
        araucaria::value::Value::USize(value) => Value::USize(value),
        araucaria::value::Value::ISize(value) => Value::ISize(value),
        araucaria::value::Value::Bool(value) => Value::Bool(value),
        araucaria::value::Value::Str(value) => Value::Str(value),
        araucaria::value::Value::Arr(value) => Value::Arr(value.into_iter().map(map_value).collect()),
        araucaria::value::Value::Obj(value) => Value::Obj(value.into_iter().map(|(k, v)| (k.clone(), map_value(v))).collect()),
    }
}

pub fn map_operand_value(value: araucaria::operation::OperandValue) -> OperandValue {
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

pub fn map_operand(operation: araucaria::operation::Operand) -> Operand {
    match operation {
        araucaria::operation::Operand::Value(operand_value) => Operand::Value(map_operand_value(operand_value)),
        araucaria::operation::Operand::FieldPath(path) => Operand::FieldPath(path.clone()),
    }
}

pub fn map_operation(operation: araucaria::operation::Operation) -> Operation {
    match operation {
        araucaria::operation::Operation::Eq(operand) => Operation::Eq(map_operand(operand)),
        araucaria::operation::Operation::Ne(operand) => Operation::Ne(map_operand(operand)),
        araucaria::operation::Operation::Gt(operand) => Operation::Gt(map_operand(operand)),
        araucaria::operation::Operation::Ge(operand) => Operation::Ge(map_operand(operand)),
        araucaria::operation::Operation::Lt(operand) => Operation::Lt(map_operand(operand)),
        araucaria::operation::Operation::Le(operand) => Operation::Le(map_operand(operand)),
        araucaria::operation::Operation::Btwn(operand_a, operand_b) => Operation::Btwn(map_operand(operand_a), map_operand(operand_b)),
    }
}

pub fn map_err(validation_err: araucaria::error::ValidationErr) -> ValidationErr {
    match validation_err {
        araucaria::error::ValidationErr::Required => ValidationErr::Required,
        araucaria::error::ValidationErr::U64 => ValidationErr::U64,
        araucaria::error::ValidationErr::I64 => ValidationErr::I64,
        araucaria::error::ValidationErr::F64 => ValidationErr::F64,
        araucaria::error::ValidationErr::USize => ValidationErr::USize,
        araucaria::error::ValidationErr::ISize => ValidationErr::ISize,
        araucaria::error::ValidationErr::Bool => ValidationErr::Bool,
        araucaria::error::ValidationErr::Str => ValidationErr::Str,
        araucaria::error::ValidationErr::Email => ValidationErr::Email,
        araucaria::error::ValidationErr::Date => ValidationErr::Date,
        araucaria::error::ValidationErr::Time => ValidationErr::Time,
        araucaria::error::ValidationErr::DateTime => ValidationErr::DateTime,
        araucaria::error::ValidationErr::Operation(operation) => ValidationErr::Operation(map_operation(operation)),
        araucaria::error::ValidationErr::BytesLen(operation) => ValidationErr::BytesLen(map_operation(operation)),
        araucaria::error::ValidationErr::CharsLen(operation) => ValidationErr::CharsLen(map_operation(operation)),
        araucaria::error::ValidationErr::GraphemesLen(operation) => ValidationErr::GraphemesLen(map_operation(operation)),
        araucaria::error::ValidationErr::LowercaseLen(operation) => ValidationErr::LowercaseLen(map_operation(operation)),
        araucaria::error::ValidationErr::UppercaseLen(operation) => ValidationErr::UppercaseLen(map_operation(operation)),
        araucaria::error::ValidationErr::NumbersLen(operation) => ValidationErr::NumbersLen(map_operation(operation)),
        araucaria::error::ValidationErr::SymbolsLen(operation) => ValidationErr::SymbolsLen(map_operation(operation)),
        araucaria::error::ValidationErr::USizeEnum(usize_values) => ValidationErr::USizeEnum(usize_values),
        araucaria::error::ValidationErr::ISizeEnum(isize_values) => ValidationErr::ISizeEnum(isize_values),
        araucaria::error::ValidationErr::StrEnum(str_values) => ValidationErr::StrEnum(str_values),
    }
}

pub fn map_schema_err(value: araucaria::error::SchemaErr) -> SchemaErr {
    match value {
        araucaria::error::SchemaErr::Arr(value) => SchemaErr::Arr(value.into_iter().map(map_err).collect()),
        araucaria::error::SchemaErr::Obj(value) => SchemaErr::Obj(value.into_iter().map(|(k, v)| (k.clone(), map_schema_err(v))).collect()),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::{Operand, OperandValue, Operation, SchemaErr, ValidationErr};

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::to_string(&SchemaErr::Obj(BTreeMap::from([
                (
                    "bool".into(),
                    SchemaErr::Arr(vec![
                        ValidationErr::Required,
                        ValidationErr::Bool,
                        ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::Bool(false))))
                    ]),
                ),
                ("u64".into(), SchemaErr::Arr(vec![ValidationErr::Required, ValidationErr::U64])),
                ("u64".into(), SchemaErr::Arr(vec![ValidationErr::Required, ValidationErr::U64])),
                ("i64".into(), SchemaErr::Arr(vec![ValidationErr::Required, ValidationErr::I64])),
                ("f64".into(), SchemaErr::Arr(vec![ValidationErr::Required, ValidationErr::F64])),
            ])))
            .unwrap(),
            r#"{"bool":["Required","Bool",{"Operation":{"Eq":{"Value":{"Bool":false}}}}],"f64":["Required","F64"],"i64":["Required","I64"],"u64":["Required","U64"]}"#.to_string()
        );
    }
}
