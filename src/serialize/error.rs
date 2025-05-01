use std::collections::BTreeMap;
use serde::{Serialize, Serializer};

use super::{to_operation, Operation};

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum EnumValues {
    USize(Vec<usize>),
    ISize(Vec<isize>),
    Str(Vec<String>),
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
    Enumerated()
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


pub fn to_validation_err(validation_err: araucaria::error::ValidationErr) -> ValidationErr {
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
        araucaria::error::ValidationErr::Operation(operation) => ValidationErr::Operation(to_operation(operation)),
        araucaria::error::ValidationErr::BytesLen(operation) => ValidationErr::BytesLen(to_operation(operation)),
        araucaria::error::ValidationErr::CharsLen(operation) => ValidationErr::CharsLen(to_operation(operation)),
        araucaria::error::ValidationErr::GraphemesLen(operation) => ValidationErr::GraphemesLen(to_operation(operation)),
        araucaria::error::ValidationErr::LowercaseLen(operation) => ValidationErr::LowercaseLen(to_operation(operation)),
        araucaria::error::ValidationErr::UppercaseLen(operation) => ValidationErr::UppercaseLen(to_operation(operation)),
        araucaria::error::ValidationErr::NumbersLen(operation) => ValidationErr::NumbersLen(to_operation(operation)),
        araucaria::error::ValidationErr::SymbolsLen(operation) => ValidationErr::SymbolsLen(to_operation(operation)),
        araucaria::error::ValidationErr::Enumerated(EnumValues) => ValidationErr::U64,
    }
}

pub fn to_schema_err(value: araucaria::error::SchemaErr) -> SchemaErr {
    match value {
        araucaria::error::SchemaErr::Arr(value) => SchemaErr::Arr(value.into_iter().map(to_validation_err).collect()),
        araucaria::error::SchemaErr::Obj(value) => SchemaErr::Obj(value.into_iter().map(|(k, v)| (k.clone(), to_schema_err(v))).collect()),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::super::{Operation, OperandValue, Operand};

    use super::{to_schema_err, to_validation_err, SchemaErr, ValidationErr};

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
