use serde::{Serialize, Serializer};
use std::collections::BTreeMap;

use super::{EnumValues, Operation, schema::to_enum_values, to_operation};

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
    Enumerated(EnumValues),
}

#[derive(Debug, PartialEq, Clone)]
pub enum SchemaErr {
    Validation(Vec<ValidationErr>),
    Arr(Vec<SchemaErr>),
    Obj(BTreeMap<String, SchemaErr>),
}

impl Serialize for SchemaErr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SchemaErr::Validation(vec) => vec.serialize(serializer),
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
        araucaria::error::ValidationErr::Enumerated(enum_values) => ValidationErr::Enumerated(to_enum_values(enum_values)),
    }
}

pub fn to_schema_err(schema_err: araucaria::error::SchemaErr) -> SchemaErr {
    match schema_err {
        araucaria::error::SchemaErr::Validation(v) => SchemaErr::Validation(v.into_iter().map(to_validation_err).collect()),
        araucaria::error::SchemaErr::Arr(arr) => SchemaErr::Arr(arr.into_iter().map(to_schema_err).collect()),
        araucaria::error::SchemaErr::Obj(obj) => SchemaErr::Obj(obj.into_iter().map(|(k, v)| (k.clone(), to_schema_err(v))).collect()),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::sync::LazyLock;

    use super::super::{EnumValues, Operand, OperandValue, Operation};

    use super::{SchemaErr, ValidationErr, to_schema_err, to_validation_err};

    static USIZE_VALUES: LazyLock<Vec<usize>> = LazyLock::new(|| vec![0, 3, 6, 9, 12, 15, 18]);
    static ISIZE_VALUES: LazyLock<Vec<isize>> = LazyLock::new(|| vec![0, -3, 6, -9, 12, -15]);
    static STR_VALUES: LazyLock<Vec<String>> = LazyLock::new(|| vec!["ROCK".into(), "PAPER".into(), "SCISSORS".into()]);

    static FIELD_LEN_1: &str = "a.len";
    static FIELD_LEN_2: &str = "b.len";
    static FIELD_LEN_3: &str = "c.len";
    static FIELD_LEN_4: &str = "d.len";

    static ARAUCARIA_ENUM_USIZE: LazyLock<araucaria::schema::EnumValues> =
        LazyLock::new(|| araucaria::schema::EnumValues::USize(USIZE_VALUES.clone()));
    static ARAUCARIA_ENUM_ISIZE: LazyLock<araucaria::schema::EnumValues> =
        LazyLock::new(|| araucaria::schema::EnumValues::ISize(ISIZE_VALUES.clone()));
    static ARAUCARIA_ENUM_STR: LazyLock<araucaria::schema::EnumValues> = LazyLock::new(|| araucaria::schema::EnumValues::Str(STR_VALUES.clone()));

    const ARAUCARIA_OPERAND_U64: araucaria::operation::Operand = araucaria::operation::Operand::Value(araucaria::operation::OperandValue::U64(12));
    const ARAUCARIA_OPERAND_USIZE_1: araucaria::operation::Operand =
        araucaria::operation::Operand::Value(araucaria::operation::OperandValue::USize(84));
    const ARAUCARIA_OPERAND_USIZE_2: araucaria::operation::Operand =
        araucaria::operation::Operand::Value(araucaria::operation::OperandValue::USize(51));
    const ARAUCARIA_OPERAND_USIZE_3: araucaria::operation::Operand =
        araucaria::operation::Operand::Value(araucaria::operation::OperandValue::USize(94));
    const ARAUCARIA_OPERAND_USIZE_4: araucaria::operation::Operand =
        araucaria::operation::Operand::Value(araucaria::operation::OperandValue::USize(46));
    static ARAUCARIA_OPERAND_FIELD_1: LazyLock<araucaria::operation::Operand> =
        LazyLock::new(|| araucaria::operation::Operand::FieldPath(FIELD_LEN_1.into()));
    static ARAUCARIA_OPERAND_FIELD_2: LazyLock<araucaria::operation::Operand> =
        LazyLock::new(|| araucaria::operation::Operand::FieldPath(FIELD_LEN_2.into()));
    static ARAUCARIA_OPERAND_FIELD_3: LazyLock<araucaria::operation::Operand> =
        LazyLock::new(|| araucaria::operation::Operand::FieldPath(FIELD_LEN_3.into()));
    static ARAUCARIA_OPERAND_FIELD_4: LazyLock<araucaria::operation::Operand> =
        LazyLock::new(|| araucaria::operation::Operand::FieldPath(FIELD_LEN_4.into()));

    const ARAUCARIA_OPERATION_EQ: araucaria::operation::Operation = araucaria::operation::Operation::Eq(ARAUCARIA_OPERAND_U64);
    const ARAUCARIA_OPERATION_LEN_EQ: araucaria::operation::Operation = araucaria::operation::Operation::Eq(ARAUCARIA_OPERAND_USIZE_1);
    const ARAUCARIA_OPERATION_LEN_NE: araucaria::operation::Operation = araucaria::operation::Operation::Ne(ARAUCARIA_OPERAND_USIZE_2);
    const ARAUCARIA_OPERATION_LEN_GT: araucaria::operation::Operation = araucaria::operation::Operation::Gt(ARAUCARIA_OPERAND_USIZE_3);
    const ARAUCARIA_OPERATION_LEN_GE: araucaria::operation::Operation = araucaria::operation::Operation::Ge(ARAUCARIA_OPERAND_USIZE_4);
    static ARAUCARIA_OPERATION_LEN_LT: LazyLock<araucaria::operation::Operation> =
        LazyLock::new(|| araucaria::operation::Operation::Lt(ARAUCARIA_OPERAND_FIELD_1.clone()));
    static ARAUCARIA_OPERATION_LEN_LE: LazyLock<araucaria::operation::Operation> =
        LazyLock::new(|| araucaria::operation::Operation::Le(ARAUCARIA_OPERAND_FIELD_2.clone()));
    static ARAUCARIA_OPERATION_LEN_BTWN: LazyLock<araucaria::operation::Operation> =
        LazyLock::new(|| araucaria::operation::Operation::Btwn(ARAUCARIA_OPERAND_FIELD_3.clone(), ARAUCARIA_OPERAND_FIELD_4.clone()));

    const ARAUCARIA_REQUIRED: araucaria::error::ValidationErr = araucaria::error::ValidationErr::Required;
    const ARAUCARIA_U64: araucaria::error::ValidationErr = araucaria::error::ValidationErr::U64;
    const ARAUCARIA_I64: araucaria::error::ValidationErr = araucaria::error::ValidationErr::I64;
    const ARAUCARIA_F64: araucaria::error::ValidationErr = araucaria::error::ValidationErr::F64;
    const ARAUCARIA_USIZE: araucaria::error::ValidationErr = araucaria::error::ValidationErr::USize;
    const ARAUCARIA_ISIZE: araucaria::error::ValidationErr = araucaria::error::ValidationErr::ISize;
    const ARAUCARIA_BOOL: araucaria::error::ValidationErr = araucaria::error::ValidationErr::Bool;
    const ARAUCARIA_STR: araucaria::error::ValidationErr = araucaria::error::ValidationErr::Str;
    const ARAUCARIA_EMAIL: araucaria::error::ValidationErr = araucaria::error::ValidationErr::Email;
    const ARAUCARIA_DATE: araucaria::error::ValidationErr = araucaria::error::ValidationErr::Date;
    const ARAUCARIA_TIME: araucaria::error::ValidationErr = araucaria::error::ValidationErr::Time;
    const ARAUCARIA_DATE_TIME: araucaria::error::ValidationErr = araucaria::error::ValidationErr::DateTime;
    const ARAUCARIA_OPERATION: araucaria::error::ValidationErr = araucaria::error::ValidationErr::Operation(ARAUCARIA_OPERATION_EQ);
    const ARAUCARIA_BYTES_LEN: araucaria::error::ValidationErr = araucaria::error::ValidationErr::BytesLen(ARAUCARIA_OPERATION_LEN_EQ);
    const ARAUCARIA_CHARS_LEN: araucaria::error::ValidationErr = araucaria::error::ValidationErr::CharsLen(ARAUCARIA_OPERATION_LEN_NE);
    const ARAUCARIA_GRAPHEMES_LEN: araucaria::error::ValidationErr = araucaria::error::ValidationErr::GraphemesLen(ARAUCARIA_OPERATION_LEN_GT);
    const ARAUCARIA_LOWERCASE_LEN: araucaria::error::ValidationErr = araucaria::error::ValidationErr::LowercaseLen(ARAUCARIA_OPERATION_LEN_GE);
    static ARAUCARIA_UPPERCASE_LEN: LazyLock<araucaria::error::ValidationErr> =
        LazyLock::new(|| araucaria::error::ValidationErr::UppercaseLen(ARAUCARIA_OPERATION_LEN_LT.clone()));
    static ARAUCARIA_NUMBERS_LEN: LazyLock<araucaria::error::ValidationErr> =
        LazyLock::new(|| araucaria::error::ValidationErr::NumbersLen(ARAUCARIA_OPERATION_LEN_LE.clone()));
    static ARAUCARIA_SYMBOLS_LEN: LazyLock<araucaria::error::ValidationErr> =
        LazyLock::new(|| araucaria::error::ValidationErr::SymbolsLen(ARAUCARIA_OPERATION_LEN_BTWN.clone()));
    static ARAUCARIA_ERR_ENUM_USIZE: LazyLock<araucaria::error::ValidationErr> =
        LazyLock::new(|| araucaria::error::ValidationErr::Enumerated(ARAUCARIA_ENUM_USIZE.clone()));
    static ARAUCARIA_ERR_ENUM_ISIZE: LazyLock<araucaria::error::ValidationErr> =
        LazyLock::new(|| araucaria::error::ValidationErr::Enumerated(ARAUCARIA_ENUM_ISIZE.clone()));
    static ARAUCARIA_ERR_ENUM_STR: LazyLock<araucaria::error::ValidationErr> =
        LazyLock::new(|| araucaria::error::ValidationErr::Enumerated(ARAUCARIA_ENUM_STR.clone()));

    static ENUM_USIZE: LazyLock<EnumValues> = LazyLock::new(|| EnumValues::USize(USIZE_VALUES.clone()));
    static ENUM_ISIZE: LazyLock<EnumValues> = LazyLock::new(|| EnumValues::ISize(ISIZE_VALUES.clone()));
    static ENUM_STR: LazyLock<EnumValues> = LazyLock::new(|| EnumValues::Str(STR_VALUES.clone()));

    const OPERAND_U64: Operand = Operand::Value(OperandValue::U64(12));
    const OPERAND_USIZE_1: Operand = Operand::Value(OperandValue::USize(84));
    const OPERAND_USIZE_2: Operand = Operand::Value(OperandValue::USize(51));
    const OPERAND_USIZE_3: Operand = Operand::Value(OperandValue::USize(94));
    const OPERAND_USIZE_4: Operand = Operand::Value(OperandValue::USize(46));
    static OPERAND_FIELD_1: LazyLock<Operand> = LazyLock::new(|| Operand::FieldPath(FIELD_LEN_1.into()));
    static OPERAND_FIELD_2: LazyLock<Operand> = LazyLock::new(|| Operand::FieldPath(FIELD_LEN_2.into()));
    static OPERAND_FIELD_3: LazyLock<Operand> = LazyLock::new(|| Operand::FieldPath(FIELD_LEN_3.into()));
    static OPERAND_FIELD_4: LazyLock<Operand> = LazyLock::new(|| Operand::FieldPath(FIELD_LEN_4.into()));

    const OPERATION_EQ: Operation = Operation::Eq(OPERAND_U64);
    const OPERATION_LEN_EQ: Operation = Operation::Eq(OPERAND_USIZE_1);
    const OPERATION_LEN_NE: Operation = Operation::Ne(OPERAND_USIZE_2);
    const OPERATION_LEN_GT: Operation = Operation::Gt(OPERAND_USIZE_3);
    const OPERATION_LEN_GE: Operation = Operation::Ge(OPERAND_USIZE_4);
    static OPERATION_LEN_LT: LazyLock<Operation> = LazyLock::new(|| Operation::Lt(OPERAND_FIELD_1.clone()));
    static OPERATION_LEN_LE: LazyLock<Operation> = LazyLock::new(|| Operation::Le(OPERAND_FIELD_2.clone()));
    static OPERATION_LEN_BTWN: LazyLock<Operation> = LazyLock::new(|| Operation::Btwn(OPERAND_FIELD_3.clone(), OPERAND_FIELD_4.clone()));

    const REQUIRED: ValidationErr = ValidationErr::Required;
    const U64: ValidationErr = ValidationErr::U64;
    const I64: ValidationErr = ValidationErr::I64;
    const F64: ValidationErr = ValidationErr::F64;
    const USIZE: ValidationErr = ValidationErr::USize;
    const ISIZE: ValidationErr = ValidationErr::ISize;
    const BOOL: ValidationErr = ValidationErr::Bool;
    const STR: ValidationErr = ValidationErr::Str;
    const EMAIL: ValidationErr = ValidationErr::Email;
    const DATE: ValidationErr = ValidationErr::Date;
    const TIME: ValidationErr = ValidationErr::Time;
    const DATE_TIME: ValidationErr = ValidationErr::DateTime;
    const OPERATION: ValidationErr = ValidationErr::Operation(OPERATION_EQ);
    const BYTES_LEN: ValidationErr = ValidationErr::BytesLen(OPERATION_LEN_EQ);
    const CHARS_LEN: ValidationErr = ValidationErr::CharsLen(OPERATION_LEN_NE);
    const GRAPHEMES_LEN: ValidationErr = ValidationErr::GraphemesLen(OPERATION_LEN_GT);
    const LOWERCASE_LEN: ValidationErr = ValidationErr::LowercaseLen(OPERATION_LEN_GE);
    static UPPERCASE_LEN: LazyLock<ValidationErr> = LazyLock::new(|| ValidationErr::UppercaseLen(OPERATION_LEN_LT.clone()));
    static NUMBERS_LEN: LazyLock<ValidationErr> = LazyLock::new(|| ValidationErr::NumbersLen(OPERATION_LEN_LE.clone()));
    static SYMBOLS_LEN: LazyLock<ValidationErr> = LazyLock::new(|| ValidationErr::SymbolsLen(OPERATION_LEN_BTWN.clone()));
    static ERR_ENUM_USIZE: LazyLock<ValidationErr> = LazyLock::new(|| ValidationErr::Enumerated(ENUM_USIZE.clone()));
    static ERR_ENUM_ISIZE: LazyLock<ValidationErr> = LazyLock::new(|| ValidationErr::Enumerated(ENUM_ISIZE.clone()));
    static ERR_ENUM_STR: LazyLock<ValidationErr> = LazyLock::new(|| ValidationErr::Enumerated(ENUM_STR.clone()));

    #[test]
    fn araucaria_validation_err_to_validation_err() {
        assert_eq!(to_validation_err(ARAUCARIA_REQUIRED), REQUIRED);
        assert_eq!(to_validation_err(ARAUCARIA_U64), U64);
        assert_eq!(to_validation_err(ARAUCARIA_I64), I64);
        assert_eq!(to_validation_err(ARAUCARIA_F64), F64);
        assert_eq!(to_validation_err(ARAUCARIA_USIZE), USIZE);
        assert_eq!(to_validation_err(ARAUCARIA_ISIZE), ISIZE);
        assert_eq!(to_validation_err(ARAUCARIA_BOOL), BOOL);
        assert_eq!(to_validation_err(ARAUCARIA_STR), STR);
        assert_eq!(to_validation_err(ARAUCARIA_EMAIL), EMAIL);
        assert_eq!(to_validation_err(ARAUCARIA_DATE), DATE);
        assert_eq!(to_validation_err(ARAUCARIA_TIME), TIME);
        assert_eq!(to_validation_err(ARAUCARIA_DATE_TIME), DATE_TIME);
        assert_eq!(to_validation_err(ARAUCARIA_OPERATION), OPERATION);
        assert_eq!(to_validation_err(ARAUCARIA_BYTES_LEN), BYTES_LEN);
        assert_eq!(to_validation_err(ARAUCARIA_CHARS_LEN), CHARS_LEN);
        assert_eq!(to_validation_err(ARAUCARIA_GRAPHEMES_LEN), GRAPHEMES_LEN);
        assert_eq!(to_validation_err(ARAUCARIA_LOWERCASE_LEN), LOWERCASE_LEN);
        assert_eq!(to_validation_err(ARAUCARIA_UPPERCASE_LEN.clone()), UPPERCASE_LEN.clone());
        assert_eq!(to_validation_err(ARAUCARIA_NUMBERS_LEN.clone()), NUMBERS_LEN.clone());
        assert_eq!(to_validation_err(ARAUCARIA_SYMBOLS_LEN.clone()), SYMBOLS_LEN.clone());
        assert_eq!(to_validation_err(ARAUCARIA_ERR_ENUM_USIZE.clone()), ERR_ENUM_USIZE.clone());
        assert_eq!(to_validation_err(ARAUCARIA_ERR_ENUM_ISIZE.clone()), ERR_ENUM_ISIZE.clone());
        assert_eq!(to_validation_err(ARAUCARIA_ERR_ENUM_STR.clone()), ERR_ENUM_STR.clone());
    }

    #[test]
    fn araucaria_schema_err_to_schema_err_validation() {
        let araucaria_schema_err_validation = araucaria::error::SchemaErr::from([
            ARAUCARIA_REQUIRED,
            ARAUCARIA_STR,
            ARAUCARIA_OPERATION,
            ARAUCARIA_BYTES_LEN,
            ARAUCARIA_CHARS_LEN,
            ARAUCARIA_GRAPHEMES_LEN,
            ARAUCARIA_LOWERCASE_LEN,
            ARAUCARIA_UPPERCASE_LEN.clone(),
            ARAUCARIA_NUMBERS_LEN.clone(),
            ARAUCARIA_SYMBOLS_LEN.clone(),
        ]);
        let schema_err_validation = SchemaErr::Validation(vec![
            REQUIRED,
            STR,
            OPERATION,
            BYTES_LEN,
            CHARS_LEN,
            GRAPHEMES_LEN,
            LOWERCASE_LEN,
            UPPERCASE_LEN.clone(),
            NUMBERS_LEN.clone(),
            SYMBOLS_LEN.clone(),
        ]);
        assert_eq!(to_schema_err(araucaria_schema_err_validation), schema_err_validation);
    }

    #[test]
    fn araucaria_schema_err_to_schema_err_arr() {
        let araucaria_schema_err_arr = araucaria::error::SchemaErr::from([
            araucaria::error::SchemaErr::from([
                araucaria::error::SchemaErr::from([ARAUCARIA_REQUIRED, ARAUCARIA_USIZE, ARAUCARIA_ERR_ENUM_USIZE.clone()]),
                araucaria::error::SchemaErr::from([ARAUCARIA_REQUIRED, ARAUCARIA_ISIZE, ARAUCARIA_ERR_ENUM_ISIZE.clone()]),
                araucaria::error::SchemaErr::from([ARAUCARIA_REQUIRED, ARAUCARIA_STR, ARAUCARIA_ERR_ENUM_STR.clone()]),
            ]),
            araucaria::error::SchemaErr::from([
                araucaria::error::SchemaErr::from([ARAUCARIA_U64]),
                araucaria::error::SchemaErr::from([ARAUCARIA_I64]),
                araucaria::error::SchemaErr::from([ARAUCARIA_F64]),
            ]),
            araucaria::error::SchemaErr::from([
                araucaria::error::SchemaErr::from([ARAUCARIA_DATE]),
                araucaria::error::SchemaErr::from([ARAUCARIA_TIME]),
                araucaria::error::SchemaErr::from([ARAUCARIA_DATE_TIME]),
            ]),
        ]);
        let schema_err_arr = SchemaErr::Arr(vec![
            SchemaErr::Arr(vec![
                SchemaErr::Validation(vec![REQUIRED, USIZE, ERR_ENUM_USIZE.clone()]),
                SchemaErr::Validation(vec![REQUIRED, ISIZE, ERR_ENUM_ISIZE.clone()]),
                SchemaErr::Validation(vec![REQUIRED, STR, ERR_ENUM_STR.clone()]),
            ]),
            SchemaErr::Arr(vec![SchemaErr::Validation(vec![U64]), SchemaErr::Validation(vec![I64]), SchemaErr::Validation(vec![F64])]),
            SchemaErr::Arr(vec![SchemaErr::Validation(vec![DATE]), SchemaErr::Validation(vec![TIME]), SchemaErr::Validation(vec![DATE_TIME])]),
        ]);
        assert_eq!(to_schema_err(araucaria_schema_err_arr), schema_err_arr);
    }

    #[test]
    fn araucaria_schema_err_to_schema_err_obj() {
        let araucaria_schema_err_obj = araucaria::error::SchemaErr::from([
            (
                "enum".into(),
                araucaria::error::SchemaErr::from([
                    araucaria::error::SchemaErr::from([ARAUCARIA_REQUIRED, ARAUCARIA_USIZE, ARAUCARIA_ERR_ENUM_USIZE.clone()]),
                    araucaria::error::SchemaErr::from([ARAUCARIA_REQUIRED, ARAUCARIA_ISIZE, ARAUCARIA_ERR_ENUM_ISIZE.clone()]),
                    araucaria::error::SchemaErr::from([ARAUCARIA_REQUIRED, ARAUCARIA_STR, ARAUCARIA_ERR_ENUM_STR.clone()]),
                ]),
            ),
            (
                "num".into(),
                araucaria::error::SchemaErr::from([
                    araucaria::error::SchemaErr::from([ARAUCARIA_U64]),
                    araucaria::error::SchemaErr::from([ARAUCARIA_I64]),
                    araucaria::error::SchemaErr::from([ARAUCARIA_F64]),
                ]),
            ),
            (
                "date/time".into(),
                araucaria::error::SchemaErr::from([
                    araucaria::error::SchemaErr::from([ARAUCARIA_DATE]),
                    araucaria::error::SchemaErr::from([ARAUCARIA_TIME]),
                    araucaria::error::SchemaErr::from([ARAUCARIA_DATE_TIME]),
                ]),
            ),
        ]);
        let schema_err_obj = SchemaErr::Obj(BTreeMap::from([
            (
                "enum".into(),
                SchemaErr::Arr(vec![
                    SchemaErr::Validation(vec![REQUIRED, USIZE, ERR_ENUM_USIZE.clone()]),
                    SchemaErr::Validation(vec![REQUIRED, ISIZE, ERR_ENUM_ISIZE.clone()]),
                    SchemaErr::Validation(vec![REQUIRED, STR, ERR_ENUM_STR.clone()]),
                ]),
            ),
            (
                "num".into(),
                SchemaErr::Arr(vec![SchemaErr::Validation(vec![U64]), SchemaErr::Validation(vec![I64]), SchemaErr::Validation(vec![F64])]),
            ),
            (
                "date/time".into(),
                SchemaErr::Arr(vec![SchemaErr::Validation(vec![DATE]), SchemaErr::Validation(vec![TIME]), SchemaErr::Validation(vec![DATE_TIME])]),
            ),
        ]));
        assert_eq!(to_schema_err(araucaria_schema_err_obj), schema_err_obj);
    }

    #[test]
    fn serialize_validation_err() {
        assert_eq!(serde_json::to_string(&REQUIRED).unwrap(), r#""Required""#.to_string());
        assert_eq!(serde_json::to_string(&U64).unwrap(), r#""U64""#.to_string());
        assert_eq!(serde_json::to_string(&I64).unwrap(), r#""I64""#.to_string());
        assert_eq!(serde_json::to_string(&F64).unwrap(), r#""F64""#.to_string());
        assert_eq!(serde_json::to_string(&USIZE).unwrap(), r#""USize""#.to_string());
        assert_eq!(serde_json::to_string(&ISIZE).unwrap(), r#""ISize""#.to_string());
        assert_eq!(serde_json::to_string(&BOOL).unwrap(), r#""Bool""#.to_string());
        assert_eq!(serde_json::to_string(&STR).unwrap(), r#""Str""#.to_string());
        assert_eq!(serde_json::to_string(&EMAIL).unwrap(), r#""Email""#.to_string());
        assert_eq!(serde_json::to_string(&DATE).unwrap(), r#""Date""#.to_string());
        assert_eq!(serde_json::to_string(&TIME).unwrap(), r#""Time""#.to_string());
        assert_eq!(serde_json::to_string(&DATE_TIME).unwrap(), r#""DateTime""#.to_string());
        assert_eq!(serde_json::to_string(&OPERATION).unwrap(), r#"{"Operation":{"Eq":12}}"#.to_string());
        assert_eq!(serde_json::to_string(&BYTES_LEN).unwrap(), r#"{"BytesLen":{"Eq":84}}"#.to_string());
        assert_eq!(serde_json::to_string(&CHARS_LEN).unwrap(), r#"{"CharsLen":{"Ne":51}}"#.to_string());
        assert_eq!(serde_json::to_string(&GRAPHEMES_LEN).unwrap(), r#"{"GraphemesLen":{"Gt":94}}"#.to_string());
        assert_eq!(serde_json::to_string(&LOWERCASE_LEN).unwrap(), r#"{"LowercaseLen":{"Ge":46}}"#.to_string());
        assert_eq!(serde_json::to_string(&UPPERCASE_LEN.clone()).unwrap(), r#"{"UppercaseLen":{"Lt":"a.len"}}"#.to_string());
        assert_eq!(serde_json::to_string(&NUMBERS_LEN.clone()).unwrap(), r#"{"NumbersLen":{"Le":"b.len"}}"#.to_string());
        assert_eq!(serde_json::to_string(&SYMBOLS_LEN.clone()).unwrap(), r#"{"SymbolsLen":{"Btwn":["c.len","d.len"]}}"#.to_string());
        assert_eq!(serde_json::to_string(&ERR_ENUM_USIZE.clone()).unwrap(), r#"{"Enumerated":[0,3,6,9,12,15,18]}"#.to_string());
        assert_eq!(serde_json::to_string(&ERR_ENUM_ISIZE.clone()).unwrap(), r#"{"Enumerated":[0,-3,6,-9,12,-15]}"#.to_string());
        assert_eq!(serde_json::to_string(&ERR_ENUM_STR.clone()).unwrap(), r#"{"Enumerated":["ROCK","PAPER","SCISSORS"]}"#.to_string());
    }

    #[test]
    fn serialize_schema_err_validation() {
        let schema_err_validation_u64 =
            SchemaErr::Validation(vec![REQUIRED, U64, ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::U64(73))))]);
        let schema_err_validation_i64 =
            SchemaErr::Validation(vec![REQUIRED, I64, ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::I64(-84))))]);
        let schema_err_validation_f64 =
            SchemaErr::Validation(vec![REQUIRED, F64, ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::F64(-28.75))))]);
        let schema_err_validation_usize =
            SchemaErr::Validation(vec![REQUIRED, USIZE, ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::USize(92))))]);
        let schema_err_validation_isize =
            SchemaErr::Validation(vec![REQUIRED, ISIZE, ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::ISize(42))))]);
        let schema_err_validation_bool =
            SchemaErr::Validation(vec![REQUIRED, BOOL, ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::Bool(false))))]);
        let schema_err_validation_str = SchemaErr::Validation(vec![
            REQUIRED,
            STR,
            ValidationErr::Operation(Operation::Btwn(
                Operand::Value(OperandValue::Str("Lua".into())),
                Operand::Value(OperandValue::Str("Saturno".into())),
            )),
        ]);
        assert_eq!(serde_json::to_string(&schema_err_validation_u64).unwrap(), r#"["Required","U64",{"Operation":{"Eq":73}}]"#.to_string());
        assert_eq!(serde_json::to_string(&schema_err_validation_i64).unwrap(), r#"["Required","I64",{"Operation":{"Ne":-84}}]"#.to_string());
        assert_eq!(serde_json::to_string(&schema_err_validation_f64).unwrap(), r#"["Required","F64",{"Operation":{"Gt":-28.75}}]"#.to_string());
        assert_eq!(serde_json::to_string(&schema_err_validation_usize).unwrap(), r#"["Required","USize",{"Operation":{"Ge":92}}]"#.to_string());
        assert_eq!(serde_json::to_string(&schema_err_validation_isize).unwrap(), r#"["Required","ISize",{"Operation":{"Lt":42}}]"#.to_string());
        assert_eq!(serde_json::to_string(&schema_err_validation_bool).unwrap(), r#"["Required","Bool",{"Operation":{"Le":false}}]"#.to_string());
        assert_eq!(
            serde_json::to_string(&schema_err_validation_str).unwrap(),
            r#"["Required","Str",{"Operation":{"Btwn":["Lua","Saturno"]}}]"#.to_string()
        );
    }

    #[test]
    fn serialize_schema_err_arr() {
        let schema_err = SchemaErr::Arr(vec![
            SchemaErr::Validation(vec![REQUIRED, BOOL]),
            SchemaErr::Validation(vec![REQUIRED, U64]),
            SchemaErr::Validation(vec![REQUIRED, I64]),
            SchemaErr::Validation(vec![REQUIRED, F64]),
        ]);
        assert_eq!(
            serde_json::to_string(&schema_err).unwrap(),
            r#"[["Required","Bool"],["Required","U64"],["Required","I64"],["Required","F64"]]"#.to_string()
        );
    }

    #[test]
    fn serialize_schema_err_obj() {
        let schema_err = SchemaErr::Obj(BTreeMap::from([
            ("bool".into(), SchemaErr::Validation(vec![REQUIRED, BOOL])),
            ("u64".into(), SchemaErr::Validation(vec![REQUIRED, U64])),
            ("i64".into(), SchemaErr::Validation(vec![REQUIRED, I64])),
            ("f64".into(), SchemaErr::Validation(vec![REQUIRED, F64])),
        ]));
        assert_eq!(
            serde_json::to_string(&schema_err).unwrap(),
            r#"{"bool":["Required","Bool"],"f64":["Required","F64"],"i64":["Required","I64"],"u64":["Required","U64"]}"#.to_string()
        );
    }

    #[test]
    fn serialize_schema_err_obj_nested() {
        let schema_err = SchemaErr::Obj(BTreeMap::from([(
            "user".into(),
            SchemaErr::Obj(BTreeMap::from([(
                "name".into(),
                SchemaErr::Validation(vec![
                    ValidationErr::Required,
                    ValidationErr::Str,
                    ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::Str("Joãozinho".into())))),
                ]),
            )])),
        )]));
        assert_eq!(
            serde_json::to_string(&schema_err).unwrap(),
            r#"{"user":{"name":["Required","Str",{"Operation":{"Eq":"Joãozinho"}}]}}"#.to_string()
        );
    }
}
