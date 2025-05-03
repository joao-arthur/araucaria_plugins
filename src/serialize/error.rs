use serde::{Serialize, Serializer};
use std::collections::BTreeMap;

use super::{EnumValues, Operation, to_operation, validation::to_enum_values};

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

    use super::super::{EnumValues, Operand, OperandValue, Operation};

    use super::{SchemaErr, ValidationErr, to_schema_err, to_validation_err};

    #[test]
    fn araucaria_validation_err_to_validation_err() {
        let araucaria_enum_usize = araucaria::validation::EnumValues::USize(vec![0, 3, 6, 9, 12, 15, 18]);
        let araucaria_enum_isize = araucaria::validation::EnumValues::ISize(vec![0, -3, 6, -9, 12, -15]);
        let araucaria_enum_str = araucaria::validation::EnumValues::Str(vec!["PEDRA".into(), "PAPEL".into(), "TESOURA".into()]);

        let araucaria_operand_u64 = araucaria::operation::Operand::Value(araucaria::operation::OperandValue::U64(12));
        let araucaria_operand_usize_1 = araucaria::operation::Operand::Value(araucaria::operation::OperandValue::USize(84));
        let araucaria_operand_usize_2 = araucaria::operation::Operand::Value(araucaria::operation::OperandValue::USize(51));
        let araucaria_operand_usize_3 = araucaria::operation::Operand::Value(araucaria::operation::OperandValue::USize(94));
        let araucaria_operand_usize_4 = araucaria::operation::Operand::Value(araucaria::operation::OperandValue::USize(46));
        let araucaria_operand_field_1 = araucaria::operation::Operand::FieldPath("a.len".into());
        let araucaria_operand_field_2 = araucaria::operation::Operand::FieldPath("b.len".into());
        let araucaria_operand_field_3 = araucaria::operation::Operand::FieldPath("c.len".into());
        let araucaria_operand_field_4 = araucaria::operation::Operand::FieldPath("d.len".into());

        let araucaria_operation_eq = araucaria::operation::Operation::Eq(araucaria_operand_u64);
        let araucaria_operation_len_eq = araucaria::operation::Operation::Eq(araucaria_operand_usize_1);
        let araucaria_operation_len_ne = araucaria::operation::Operation::Ne(araucaria_operand_usize_2);
        let araucaria_operation_len_gt = araucaria::operation::Operation::Gt(araucaria_operand_usize_3);
        let araucaria_operation_len_ge = araucaria::operation::Operation::Ge(araucaria_operand_usize_4);
        let araucaria_operation_len_lt = araucaria::operation::Operation::Lt(araucaria_operand_field_1);
        let araucaria_operation_len_le = araucaria::operation::Operation::Le(araucaria_operand_field_2);
        let araucaria_operation_len_btwn = araucaria::operation::Operation::Btwn(araucaria_operand_field_3, araucaria_operand_field_4);

        let araucaria_err_required = araucaria::error::ValidationErr::Required;
        let araucaria_err_u64 = araucaria::error::ValidationErr::U64;
        let araucaria_err_i64 = araucaria::error::ValidationErr::I64;
        let araucaria_err_f64 = araucaria::error::ValidationErr::F64;
        let araucaria_err_usize = araucaria::error::ValidationErr::USize;
        let araucaria_err_isize = araucaria::error::ValidationErr::ISize;
        let araucaria_err_bool = araucaria::error::ValidationErr::Bool;
        let araucaria_err_str = araucaria::error::ValidationErr::Str;
        let araucaria_err_email = araucaria::error::ValidationErr::Email;
        let araucaria_err_date = araucaria::error::ValidationErr::Date;
        let araucaria_err_time = araucaria::error::ValidationErr::Time;
        let araucaria_err_datetime = araucaria::error::ValidationErr::DateTime;
        let araucaria_err_operation = araucaria::error::ValidationErr::Operation(araucaria_operation_eq);
        let araucaria_err_bytes_len = araucaria::error::ValidationErr::BytesLen(araucaria_operation_len_eq);
        let araucaria_err_chars_len = araucaria::error::ValidationErr::CharsLen(araucaria_operation_len_ne);
        let araucaria_err_graphemes_len = araucaria::error::ValidationErr::GraphemesLen(araucaria_operation_len_gt);
        let araucaria_err_lowercase_len = araucaria::error::ValidationErr::LowercaseLen(araucaria_operation_len_ge);
        let araucaria_err_uppercase_len = araucaria::error::ValidationErr::UppercaseLen(araucaria_operation_len_lt);
        let araucaria_err_numbers_len = araucaria::error::ValidationErr::NumbersLen(araucaria_operation_len_le);
        let araucaria_err_symbols_len = araucaria::error::ValidationErr::SymbolsLen(araucaria_operation_len_btwn);
        let araucaria_err_enum_usize = araucaria::error::ValidationErr::Enumerated(araucaria_enum_usize);
        let araucaria_err_enum_isize = araucaria::error::ValidationErr::Enumerated(araucaria_enum_isize);
        let araucaria_err_enum_str = araucaria::error::ValidationErr::Enumerated(araucaria_enum_str);

        let err_required = ValidationErr::Required;
        let err_u64 = ValidationErr::U64;
        let err_i64 = ValidationErr::I64;
        let err_f64 = ValidationErr::F64;
        let err_usize = ValidationErr::USize;
        let err_isize = ValidationErr::ISize;
        let err_bool = ValidationErr::Bool;
        let err_str = ValidationErr::Str;
        let err_email = ValidationErr::Email;
        let err_date = ValidationErr::Date;
        let err_time = ValidationErr::Time;
        let err_datetime = ValidationErr::DateTime;
        let err_operation = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::U64(12))));
        let err_bytes_len = ValidationErr::BytesLen(Operation::Eq(Operand::Value(OperandValue::USize(84))));
        let err_chars_len = ValidationErr::CharsLen(Operation::Ne(Operand::Value(OperandValue::USize(51))));
        let err_graphemes_len = ValidationErr::GraphemesLen(Operation::Gt(Operand::Value(OperandValue::USize(94))));
        let err_lowercase_len = ValidationErr::LowercaseLen(Operation::Ge(Operand::Value(OperandValue::USize(46))));
        let err_uppercase_len = ValidationErr::UppercaseLen(Operation::Lt(Operand::FieldPath("a.len".into())));
        let err_numbers_len = ValidationErr::NumbersLen(Operation::Le(Operand::FieldPath("b.len".into())));
        let err_symbols_len = ValidationErr::SymbolsLen(Operation::Btwn(Operand::FieldPath("c.len".into()), Operand::FieldPath("d.len".into())));
        let err_enum_usize = ValidationErr::Enumerated(EnumValues::USize(vec![0, 3, 6, 9, 12, 15, 18]));
        let err_enum_isize = ValidationErr::Enumerated(EnumValues::ISize(vec![0, -3, 6, -9, 12, -15]));
        let err_enum_str = ValidationErr::Enumerated(EnumValues::Str(vec!["PEDRA".into(), "PAPEL".into(), "TESOURA".into()]));

        assert_eq!(to_validation_err(araucaria_err_required), err_required);
        assert_eq!(to_validation_err(araucaria_err_u64), err_u64);
        assert_eq!(to_validation_err(araucaria_err_i64), err_i64);
        assert_eq!(to_validation_err(araucaria_err_f64), err_f64);
        assert_eq!(to_validation_err(araucaria_err_usize), err_usize);
        assert_eq!(to_validation_err(araucaria_err_isize), err_isize);
        assert_eq!(to_validation_err(araucaria_err_bool), err_bool);
        assert_eq!(to_validation_err(araucaria_err_str), err_str);
        assert_eq!(to_validation_err(araucaria_err_email), err_email);
        assert_eq!(to_validation_err(araucaria_err_date), err_date);
        assert_eq!(to_validation_err(araucaria_err_time), err_time);
        assert_eq!(to_validation_err(araucaria_err_datetime), err_datetime);
        assert_eq!(to_validation_err(araucaria_err_operation), err_operation);
        assert_eq!(to_validation_err(araucaria_err_bytes_len), err_bytes_len);
        assert_eq!(to_validation_err(araucaria_err_chars_len), err_chars_len);
        assert_eq!(to_validation_err(araucaria_err_graphemes_len), err_graphemes_len);
        assert_eq!(to_validation_err(araucaria_err_lowercase_len), err_lowercase_len);
        assert_eq!(to_validation_err(araucaria_err_uppercase_len), err_uppercase_len);
        assert_eq!(to_validation_err(araucaria_err_numbers_len), err_numbers_len);
        assert_eq!(to_validation_err(araucaria_err_symbols_len), err_symbols_len);
        assert_eq!(to_validation_err(araucaria_err_enum_usize), err_enum_usize);
        assert_eq!(to_validation_err(araucaria_err_enum_isize), err_enum_isize);
        assert_eq!(to_validation_err(araucaria_err_enum_str), err_enum_str);
    }

    #[test]
    fn araucaria_schema_err_to_schema_err() {
        let araucaria_enum_usize = araucaria::validation::EnumValues::USize(vec![0, 3, 6, 9, 12, 15, 18]);
        let araucaria_enum_isize = araucaria::validation::EnumValues::ISize(vec![0, -3, 6, -9, 12, -15]);
        let araucaria_enum_str = araucaria::validation::EnumValues::Str(vec!["PEDRA".into(), "PAPEL".into(), "TESOURA".into()]);

        let araucaria_operand_u64 = araucaria::operation::Operand::Value(araucaria::operation::OperandValue::U64(12));
        let araucaria_operand_usize_1 = araucaria::operation::Operand::Value(araucaria::operation::OperandValue::USize(84));
        let araucaria_operand_usize_2 = araucaria::operation::Operand::Value(araucaria::operation::OperandValue::USize(51));
        let araucaria_operand_usize_3 = araucaria::operation::Operand::Value(araucaria::operation::OperandValue::USize(94));
        let araucaria_operand_usize_4 = araucaria::operation::Operand::Value(araucaria::operation::OperandValue::USize(46));
        let araucaria_operand_field_1 = araucaria::operation::Operand::FieldPath("a.len".into());
        let araucaria_operand_field_2 = araucaria::operation::Operand::FieldPath("b.len".into());
        let araucaria_operand_field_3 = araucaria::operation::Operand::FieldPath("c.len".into());
        let araucaria_operand_field_4 = araucaria::operation::Operand::FieldPath("d.len".into());

        let araucaria_operation_eq = araucaria::operation::Operation::Eq(araucaria_operand_u64);
        let araucaria_operation_len_eq = araucaria::operation::Operation::Eq(araucaria_operand_usize_1);
        let araucaria_operation_len_ne = araucaria::operation::Operation::Ne(araucaria_operand_usize_2);
        let araucaria_operation_len_gt = araucaria::operation::Operation::Gt(araucaria_operand_usize_3);
        let araucaria_operation_len_ge = araucaria::operation::Operation::Ge(araucaria_operand_usize_4);
        let araucaria_operation_len_lt = araucaria::operation::Operation::Lt(araucaria_operand_field_1);
        let araucaria_operation_len_le = araucaria::operation::Operation::Le(araucaria_operand_field_2);
        let araucaria_operation_len_btwn = araucaria::operation::Operation::Btwn(araucaria_operand_field_3, araucaria_operand_field_4);

        let araucaria_err_required = araucaria::error::ValidationErr::Required;
        let araucaria_err_u64 = araucaria::error::ValidationErr::U64;
        let araucaria_err_i64 = araucaria::error::ValidationErr::I64;
        let araucaria_err_f64 = araucaria::error::ValidationErr::F64;
        let araucaria_err_usize = araucaria::error::ValidationErr::USize;
        let araucaria_err_isize = araucaria::error::ValidationErr::ISize;
        let araucaria_err_bool = araucaria::error::ValidationErr::Bool;
        let araucaria_err_str = araucaria::error::ValidationErr::Str;
        let araucaria_err_email = araucaria::error::ValidationErr::Email;
        let araucaria_err_date = araucaria::error::ValidationErr::Date;
        let araucaria_err_time = araucaria::error::ValidationErr::Time;
        let araucaria_err_datetime = araucaria::error::ValidationErr::DateTime;
        let araucaria_err_operation = araucaria::error::ValidationErr::Operation(araucaria_operation_eq);
        let araucaria_err_bytes_len = araucaria::error::ValidationErr::BytesLen(araucaria_operation_len_eq);
        let araucaria_err_chars_len = araucaria::error::ValidationErr::CharsLen(araucaria_operation_len_ne);
        let araucaria_err_graphemes_len = araucaria::error::ValidationErr::GraphemesLen(araucaria_operation_len_gt);
        let araucaria_err_lowercase_len = araucaria::error::ValidationErr::LowercaseLen(araucaria_operation_len_ge);
        let araucaria_err_uppercase_len = araucaria::error::ValidationErr::UppercaseLen(araucaria_operation_len_lt);
        let araucaria_err_numbers_len = araucaria::error::ValidationErr::NumbersLen(araucaria_operation_len_le);
        let araucaria_err_symbols_len = araucaria::error::ValidationErr::SymbolsLen(araucaria_operation_len_btwn);
        let araucaria_err_enum_usize = araucaria::error::ValidationErr::Enumerated(araucaria_enum_usize);
        let araucaria_err_enum_isize = araucaria::error::ValidationErr::Enumerated(araucaria_enum_isize);
        let araucaria_err_enum_str = araucaria::error::ValidationErr::Enumerated(araucaria_enum_str);

        let araucaria_schema_err_arr = araucaria::error::SchemaErr::Validation(vec![
            araucaria_err_required.clone(),
            araucaria_err_u64.clone(),
            araucaria_err_i64.clone(),
            araucaria_err_f64.clone(),
            araucaria_err_usize.clone(),
            araucaria_err_isize.clone(),
            araucaria_err_bool.clone(),
            araucaria_err_str.clone(),
            araucaria_err_email.clone(),
            araucaria_err_date.clone(),
            araucaria_err_time.clone(),
            araucaria_err_datetime.clone(),
            araucaria_err_operation.clone(),
            araucaria_err_bytes_len.clone(),
            araucaria_err_chars_len.clone(),
            araucaria_err_graphemes_len.clone(),
            araucaria_err_lowercase_len.clone(),
            araucaria_err_uppercase_len.clone(),
            araucaria_err_numbers_len.clone(),
            araucaria_err_symbols_len.clone(),
            araucaria_err_enum_usize.clone(),
            araucaria_err_enum_isize.clone(),
            araucaria_err_enum_str.clone(),
        ]);
        let araucaria_schema_err_obj = araucaria::error::SchemaErr::Obj(BTreeMap::from([
            ("required".into(), araucaria::error::SchemaErr::Validation(vec![araucaria_err_required])),
            ("u64".into(), araucaria::error::SchemaErr::Validation(vec![araucaria_err_u64])),
            ("i64".into(), araucaria::error::SchemaErr::Validation(vec![araucaria_err_i64])),
            ("f64".into(), araucaria::error::SchemaErr::Validation(vec![araucaria_err_f64])),
            ("usize".into(), araucaria::error::SchemaErr::Validation(vec![araucaria_err_usize])),
            ("isize".into(), araucaria::error::SchemaErr::Validation(vec![araucaria_err_isize])),
            ("bool".into(), araucaria::error::SchemaErr::Validation(vec![araucaria_err_bool])),
            ("str".into(), araucaria::error::SchemaErr::Validation(vec![araucaria_err_str])),
            ("email".into(), araucaria::error::SchemaErr::Validation(vec![araucaria_err_email])),
            ("date".into(), araucaria::error::SchemaErr::Validation(vec![araucaria_err_date])),
            ("time".into(), araucaria::error::SchemaErr::Validation(vec![araucaria_err_time])),
            ("datetime".into(), araucaria::error::SchemaErr::Validation(vec![araucaria_err_datetime])),
            ("operation".into(), araucaria::error::SchemaErr::Validation(vec![araucaria_err_operation])),
            ("bytes_len".into(), araucaria::error::SchemaErr::Validation(vec![araucaria_err_bytes_len])),
            ("chars_len".into(), araucaria::error::SchemaErr::Validation(vec![araucaria_err_chars_len])),
            ("graphemes_len".into(), araucaria::error::SchemaErr::Validation(vec![araucaria_err_graphemes_len])),
            ("lowercase_len".into(), araucaria::error::SchemaErr::Validation(vec![araucaria_err_lowercase_len])),
            ("uppercase_len".into(), araucaria::error::SchemaErr::Validation(vec![araucaria_err_uppercase_len])),
            ("numbers_len".into(), araucaria::error::SchemaErr::Validation(vec![araucaria_err_numbers_len])),
            ("symbols_len".into(), araucaria::error::SchemaErr::Validation(vec![araucaria_err_symbols_len])),
            ("enum_usize".into(), araucaria::error::SchemaErr::Validation(vec![araucaria_err_enum_usize])),
            ("enum_isize".into(), araucaria::error::SchemaErr::Validation(vec![araucaria_err_enum_isize])),
            ("enum_str".into(), araucaria::error::SchemaErr::Validation(vec![araucaria_err_enum_str])),
        ]));

        let err_required = ValidationErr::Required;
        let err_u64 = ValidationErr::U64;
        let err_i64 = ValidationErr::I64;
        let err_f64 = ValidationErr::F64;
        let err_usize = ValidationErr::USize;
        let err_isize = ValidationErr::ISize;
        let err_bool = ValidationErr::Bool;
        let err_str = ValidationErr::Str;
        let err_email = ValidationErr::Email;
        let err_date = ValidationErr::Date;
        let err_time = ValidationErr::Time;
        let err_datetime = ValidationErr::DateTime;
        let err_operation = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::U64(12))));
        let err_bytes_len = ValidationErr::BytesLen(Operation::Eq(Operand::Value(OperandValue::USize(84))));
        let err_chars_len = ValidationErr::CharsLen(Operation::Ne(Operand::Value(OperandValue::USize(51))));
        let err_graphemes_len = ValidationErr::GraphemesLen(Operation::Gt(Operand::Value(OperandValue::USize(94))));
        let err_lowercase_len = ValidationErr::LowercaseLen(Operation::Ge(Operand::Value(OperandValue::USize(46))));
        let err_uppercase_len = ValidationErr::UppercaseLen(Operation::Lt(Operand::FieldPath("a.len".into())));
        let err_numbers_len = ValidationErr::NumbersLen(Operation::Le(Operand::FieldPath("b.len".into())));
        let err_symbols_len = ValidationErr::SymbolsLen(Operation::Btwn(Operand::FieldPath("c.len".into()), Operand::FieldPath("d.len".into())));
        let err_enum_usize = ValidationErr::Enumerated(EnumValues::USize(vec![0, 3, 6, 9, 12, 15, 18]));
        let err_enum_isize = ValidationErr::Enumerated(EnumValues::ISize(vec![0, -3, 6, -9, 12, -15]));
        let err_enum_str = ValidationErr::Enumerated(EnumValues::Str(vec!["PEDRA".into(), "PAPEL".into(), "TESOURA".into()]));

        let schema_err_arr = SchemaErr::Validation(vec![
            err_required.clone(),
            err_u64.clone(),
            err_i64.clone(),
            err_f64.clone(),
            err_usize.clone(),
            err_isize.clone(),
            err_bool.clone(),
            err_str.clone(),
            err_email.clone(),
            err_date.clone(),
            err_time.clone(),
            err_datetime.clone(),
            err_operation.clone(),
            err_bytes_len.clone(),
            err_chars_len.clone(),
            err_graphemes_len.clone(),
            err_lowercase_len.clone(),
            err_uppercase_len.clone(),
            err_numbers_len.clone(),
            err_symbols_len.clone(),
            err_enum_usize.clone(),
            err_enum_isize.clone(),
            err_enum_str.clone(),
        ]);
        let schema_err_obj = SchemaErr::Obj(BTreeMap::from([
            ("required".into(), SchemaErr::Validation(vec![err_required])),
            ("u64".into(), SchemaErr::Validation(vec![err_u64])),
            ("i64".into(), SchemaErr::Validation(vec![err_i64])),
            ("f64".into(), SchemaErr::Validation(vec![err_f64])),
            ("usize".into(), SchemaErr::Validation(vec![err_usize])),
            ("isize".into(), SchemaErr::Validation(vec![err_isize])),
            ("bool".into(), SchemaErr::Validation(vec![err_bool])),
            ("str".into(), SchemaErr::Validation(vec![err_str])),
            ("email".into(), SchemaErr::Validation(vec![err_email])),
            ("date".into(), SchemaErr::Validation(vec![err_date])),
            ("time".into(), SchemaErr::Validation(vec![err_time])),
            ("datetime".into(), SchemaErr::Validation(vec![err_datetime])),
            ("operation".into(), SchemaErr::Validation(vec![err_operation])),
            ("bytes_len".into(), SchemaErr::Validation(vec![err_bytes_len])),
            ("chars_len".into(), SchemaErr::Validation(vec![err_chars_len])),
            ("graphemes_len".into(), SchemaErr::Validation(vec![err_graphemes_len])),
            ("lowercase_len".into(), SchemaErr::Validation(vec![err_lowercase_len])),
            ("uppercase_len".into(), SchemaErr::Validation(vec![err_uppercase_len])),
            ("numbers_len".into(), SchemaErr::Validation(vec![err_numbers_len])),
            ("symbols_len".into(), SchemaErr::Validation(vec![err_symbols_len])),
            ("enum_usize".into(), SchemaErr::Validation(vec![err_enum_usize])),
            ("enum_isize".into(), SchemaErr::Validation(vec![err_enum_isize])),
            ("enum_str".into(), SchemaErr::Validation(vec![err_enum_str])),
        ]));

        assert_eq!(to_schema_err(araucaria_schema_err_arr), schema_err_arr);
        assert_eq!(to_schema_err(araucaria_schema_err_obj), schema_err_obj);
    }

    #[test]
    fn serialize_validation_err() {
        let err_operation = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::U64(12))));
        let err_bytes_len = ValidationErr::BytesLen(Operation::Eq(Operand::Value(OperandValue::USize(84))));
        let err_chars_len = ValidationErr::CharsLen(Operation::Ne(Operand::Value(OperandValue::USize(51))));
        let err_graphemes_len = ValidationErr::GraphemesLen(Operation::Gt(Operand::Value(OperandValue::USize(94))));
        let err_lowercase_len = ValidationErr::LowercaseLen(Operation::Ge(Operand::Value(OperandValue::USize(46))));
        let err_uppercase_len = ValidationErr::UppercaseLen(Operation::Lt(Operand::FieldPath("a.len".into())));
        let err_numbers_len = ValidationErr::NumbersLen(Operation::Le(Operand::FieldPath("b.len".into())));
        let err_symbols_len = ValidationErr::SymbolsLen(Operation::Btwn(Operand::FieldPath("c.len".into()), Operand::FieldPath("d.len".into())));
        let err_enum_usize = ValidationErr::Enumerated(EnumValues::USize(vec![0, 3, 6, 9, 12, 15, 18]));
        let err_enum_isize = ValidationErr::Enumerated(EnumValues::ISize(vec![0, -3, 6, -9, 12, -15]));
        let err_enum_str = ValidationErr::Enumerated(EnumValues::Str(vec!["PEDRA".into(), "PAPEL".into(), "TESOURA".into()]));

        assert_eq!(serde_json::to_string(&ValidationErr::Required).unwrap(), r#""Required""#.to_string());
        assert_eq!(serde_json::to_string(&ValidationErr::U64).unwrap(), r#""U64""#.to_string());
        assert_eq!(serde_json::to_string(&ValidationErr::I64).unwrap(), r#""I64""#.to_string());
        assert_eq!(serde_json::to_string(&ValidationErr::F64).unwrap(), r#""F64""#.to_string());
        assert_eq!(serde_json::to_string(&ValidationErr::USize).unwrap(), r#""USize""#.to_string());
        assert_eq!(serde_json::to_string(&ValidationErr::ISize).unwrap(), r#""ISize""#.to_string());
        assert_eq!(serde_json::to_string(&ValidationErr::Bool).unwrap(), r#""Bool""#.to_string());
        assert_eq!(serde_json::to_string(&ValidationErr::Str).unwrap(), r#""Str""#.to_string());
        assert_eq!(serde_json::to_string(&ValidationErr::Email).unwrap(), r#""Email""#.to_string());
        assert_eq!(serde_json::to_string(&ValidationErr::Date).unwrap(), r#""Date""#.to_string());
        assert_eq!(serde_json::to_string(&ValidationErr::Time).unwrap(), r#""Time""#.to_string());
        assert_eq!(serde_json::to_string(&ValidationErr::DateTime).unwrap(), r#""DateTime""#.to_string());
        assert_eq!(serde_json::to_string(&err_operation).unwrap(), r#"{"Operation":{"Eq":12}}"#.to_string());
        assert_eq!(serde_json::to_string(&err_bytes_len).unwrap(), r#"{"BytesLen":{"Eq":84}}"#.to_string());
        assert_eq!(serde_json::to_string(&err_chars_len).unwrap(), r#"{"CharsLen":{"Ne":51}}"#.to_string());
        assert_eq!(serde_json::to_string(&err_graphemes_len).unwrap(), r#"{"GraphemesLen":{"Gt":94}}"#.to_string());
        assert_eq!(serde_json::to_string(&err_lowercase_len).unwrap(), r#"{"LowercaseLen":{"Ge":46}}"#.to_string());
        assert_eq!(serde_json::to_string(&err_uppercase_len).unwrap(), r#"{"UppercaseLen":{"Lt":"a.len"}}"#.to_string());
        assert_eq!(serde_json::to_string(&err_numbers_len).unwrap(), r#"{"NumbersLen":{"Le":"b.len"}}"#.to_string());
        assert_eq!(serde_json::to_string(&err_symbols_len).unwrap(), r#"{"SymbolsLen":{"Btwn":["c.len","d.len"]}}"#.to_string());
        assert_eq!(serde_json::to_string(&err_enum_usize).unwrap(), r#"{"Enumerated":[0,3,6,9,12,15,18]}"#.to_string());
        assert_eq!(serde_json::to_string(&err_enum_isize).unwrap(), r#"{"Enumerated":[0,-3,6,-9,12,-15]}"#.to_string());
        assert_eq!(serde_json::to_string(&err_enum_str).unwrap(), r#"{"Enumerated":["PEDRA","PAPEL","TESOURA"]}"#.to_string());
    }

    #[test]
    fn serialize_schema_err_arr() {
        let schema_err_arr = SchemaErr::Validation(vec![
            ValidationErr::Required,
            ValidationErr::U64,
            ValidationErr::I64,
            ValidationErr::F64,
            ValidationErr::USize,
            ValidationErr::ISize,
        ]);
        let schema_err_arr_u64 = SchemaErr::Validation(vec![
            ValidationErr::Required,
            ValidationErr::U64,
            ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::U64(73)))),
        ]);
        let schema_err_arr_i64 = SchemaErr::Validation(vec![
            ValidationErr::Required,
            ValidationErr::I64,
            ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::I64(-84)))),
        ]);
        let schema_err_arr_f64 = SchemaErr::Validation(vec![
            ValidationErr::Required,
            ValidationErr::F64,
            ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::F64(-28.75)))),
        ]);
        let schema_err_arr_usize = SchemaErr::Validation(vec![
            ValidationErr::Required,
            ValidationErr::USize,
            ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::USize(92)))),
        ]);
        let schema_err_arr_isize = SchemaErr::Validation(vec![
            ValidationErr::Required,
            ValidationErr::ISize,
            ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::ISize(42)))),
        ]);
        let schema_err_arr_bool = SchemaErr::Validation(vec![
            ValidationErr::Required,
            ValidationErr::Bool,
            ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::Bool(false)))),
        ]);
        let schema_err_arr_str = SchemaErr::Validation(vec![
            ValidationErr::Required,
            ValidationErr::Str,
            ValidationErr::Operation(Operation::Btwn(
                Operand::Value(OperandValue::Str("Lua".into())),
                Operand::Value(OperandValue::Str("Saturno".into())),
            )),
        ]);
        assert_eq!(serde_json::to_string(&schema_err_arr).unwrap(), r#"["Required","U64","I64","F64","USize","ISize"]"#.to_string());
        assert_eq!(serde_json::to_string(&schema_err_arr_u64).unwrap(), r#"["Required","U64",{"Operation":{"Eq":73}}]"#.to_string());
        assert_eq!(serde_json::to_string(&schema_err_arr_i64).unwrap(), r#"["Required","I64",{"Operation":{"Ne":-84}}]"#.to_string());
        assert_eq!(serde_json::to_string(&schema_err_arr_f64).unwrap(), r#"["Required","F64",{"Operation":{"Gt":-28.75}}]"#.to_string());
        assert_eq!(serde_json::to_string(&schema_err_arr_usize).unwrap(), r#"["Required","USize",{"Operation":{"Ge":92}}]"#.to_string());
        assert_eq!(serde_json::to_string(&schema_err_arr_isize).unwrap(), r#"["Required","ISize",{"Operation":{"Lt":42}}]"#.to_string());
        assert_eq!(serde_json::to_string(&schema_err_arr_bool).unwrap(), r#"["Required","Bool",{"Operation":{"Le":false}}]"#.to_string());
        assert_eq!(serde_json::to_string(&schema_err_arr_str).unwrap(), r#"["Required","Str",{"Operation":{"Btwn":["Lua","Saturno"]}}]"#.to_string());
    }

    #[test]
    fn serialize_schema_err_obj() {
        let bool_op = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::Bool(false))));
        let u64_op = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::U64(73))));
        let i64_op = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::I64(-84))));
        let f64_op = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::F64(-28.75))));
        let schema_err = SchemaErr::Obj(BTreeMap::from([
            ("bool".into(), SchemaErr::Validation(vec![ValidationErr::Required, ValidationErr::Bool])),
            ("u64".into(), SchemaErr::Validation(vec![ValidationErr::Required, ValidationErr::U64])),
            ("i64".into(), SchemaErr::Validation(vec![ValidationErr::Required, ValidationErr::I64])),
            ("f64".into(), SchemaErr::Validation(vec![ValidationErr::Required, ValidationErr::F64])),
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
