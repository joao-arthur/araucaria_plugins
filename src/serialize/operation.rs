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

    use super::{Operand, OperandValue, Operation, to_operand, to_operand_value, to_operation};

    #[test]
    fn araucaria_operand_value_to_operand_value() {
        assert_eq!(to_operand_value(araucaria::operation::OperandValue::U64(12)), OperandValue::U64(12));
        assert_eq!(to_operand_value(araucaria::operation::OperandValue::I64(-34)), OperandValue::I64(-34));
        assert_eq!(to_operand_value(araucaria::operation::OperandValue::F64(-64.5)), OperandValue::F64(-64.5));
        assert_eq!(to_operand_value(araucaria::operation::OperandValue::USize(84)), OperandValue::USize(84));
        assert_eq!(to_operand_value(araucaria::operation::OperandValue::ISize(-79)), OperandValue::ISize(-79));
        assert_eq!(to_operand_value(araucaria::operation::OperandValue::Bool(false)), OperandValue::Bool(false));
        assert_eq!(to_operand_value(araucaria::operation::OperandValue::Str("I saw you".into())), OperandValue::Str("I saw you".into()));
    }

    #[test]
    fn araucaria_operand_to_operand() {
        let araucaria_operand_u64 = araucaria::operation::Operand::Value(araucaria::operation::OperandValue::U64(12));
        let araucaria_operand_i64 = araucaria::operation::Operand::Value(araucaria::operation::OperandValue::I64(-34));
        let araucaria_operand_f64 = araucaria::operation::Operand::Value(araucaria::operation::OperandValue::F64(-64.5));
        let araucaria_operand_usize = araucaria::operation::Operand::Value(araucaria::operation::OperandValue::USize(84));
        let araucaria_operand_isize = araucaria::operation::Operand::Value(araucaria::operation::OperandValue::ISize(-79));
        let araucaria_operand_bool = araucaria::operation::Operand::Value(araucaria::operation::OperandValue::Bool(false));
        let araucaria_operand_str = araucaria::operation::Operand::Value(araucaria::operation::OperandValue::Str("I saw you".into()));
        let araucaria_operand_field = araucaria::operation::Operand::FieldPath("info.data.details.value".into());

        assert_eq!(to_operand(araucaria_operand_u64), Operand::Value(OperandValue::U64(12)));
        assert_eq!(to_operand(araucaria_operand_i64), Operand::Value(OperandValue::I64(-34)));
        assert_eq!(to_operand(araucaria_operand_f64), Operand::Value(OperandValue::F64(-64.5)));
        assert_eq!(to_operand(araucaria_operand_usize), Operand::Value(OperandValue::USize(84)));
        assert_eq!(to_operand(araucaria_operand_isize), Operand::Value(OperandValue::ISize(-79)));
        assert_eq!(to_operand(araucaria_operand_bool), Operand::Value(OperandValue::Bool(false)));
        assert_eq!(to_operand(araucaria_operand_str), Operand::Value(OperandValue::Str("I saw you".into())));
        assert_eq!(to_operand(araucaria_operand_field), Operand::FieldPath("info.data.details.value".into()));
    }

    #[test]
    fn araucaria_operation_to_operation() {
        let araucaria_operand_u64 = araucaria::operation::Operand::Value(araucaria::operation::OperandValue::U64(12));
        let araucaria_operand_i64 = araucaria::operation::Operand::Value(araucaria::operation::OperandValue::I64(-34));
        let araucaria_operand_f64 = araucaria::operation::Operand::Value(araucaria::operation::OperandValue::F64(-64.5));
        let araucaria_operand_usize = araucaria::operation::Operand::Value(araucaria::operation::OperandValue::USize(84));
        let araucaria_operand_isize = araucaria::operation::Operand::Value(araucaria::operation::OperandValue::ISize(-79));
        let araucaria_operand_bool = araucaria::operation::Operand::Value(araucaria::operation::OperandValue::Bool(false));
        let araucaria_operand_str = araucaria::operation::Operand::Value(araucaria::operation::OperandValue::Str("Pink moon".into()));
        let araucaria_operand_field = araucaria::operation::Operand::FieldPath("a.b.c".into());

        let araucaria_operation_eq = araucaria::operation::Operation::Eq(araucaria_operand_u64);
        let araucaria_operation_ne = araucaria::operation::Operation::Ne(araucaria_operand_i64);
        let araucaria_operation_gt = araucaria::operation::Operation::Gt(araucaria_operand_f64);
        let araucaria_operation_ge = araucaria::operation::Operation::Ge(araucaria_operand_usize);
        let araucaria_operation_lt = araucaria::operation::Operation::Lt(araucaria_operand_isize);
        let araucaria_operation_le = araucaria::operation::Operation::Le(araucaria_operand_bool);
        let araucaria_operation_btwn = araucaria::operation::Operation::Btwn(araucaria_operand_str, araucaria_operand_field);

        let operation_btwn = Operation::Btwn(Operand::Value(OperandValue::Str("Pink moon".into())), Operand::FieldPath("a.b.c".into()));

        assert_eq!(to_operation(araucaria_operation_eq), Operation::Eq(Operand::Value(OperandValue::U64(12))));
        assert_eq!(to_operation(araucaria_operation_ne), Operation::Ne(Operand::Value(OperandValue::I64(-34))));
        assert_eq!(to_operation(araucaria_operation_gt), Operation::Gt(Operand::Value(OperandValue::F64(-64.5))));
        assert_eq!(to_operation(araucaria_operation_ge), Operation::Ge(Operand::Value(OperandValue::USize(84))));
        assert_eq!(to_operation(araucaria_operation_lt), Operation::Lt(Operand::Value(OperandValue::ISize(-79))));
        assert_eq!(to_operation(araucaria_operation_le), Operation::Le(Operand::Value(OperandValue::Bool(false))));
        assert_eq!(to_operation(araucaria_operation_btwn), operation_btwn);
    }

    #[test]
    fn serialize_operand_value() {
        assert_eq!(serde_json::to_string(&OperandValue::U64(12)).unwrap(), "12".to_string());
        assert_eq!(serde_json::to_string(&OperandValue::I64(-34)).unwrap(), "-34".to_string());
        assert_eq!(serde_json::to_string(&OperandValue::F64(-64.5)).unwrap(), "-64.5".to_string());
        assert_eq!(serde_json::to_string(&OperandValue::USize(84)).unwrap(), "84".to_string());
        assert_eq!(serde_json::to_string(&OperandValue::ISize(-79)).unwrap(), "-79".to_string());
        assert_eq!(serde_json::to_string(&OperandValue::Bool(false)).unwrap(), "false".to_string());
        assert_eq!(serde_json::to_string(&OperandValue::Str("O sol vê tudo".into())).unwrap(), r#""O sol vê tudo""#.to_string());
    }

    #[test]
    fn serialize_operand() {
        assert_eq!(serde_json::to_string(&Operand::Value(OperandValue::U64(12))).unwrap(), "12".to_string());
        assert_eq!(serde_json::to_string(&Operand::Value(OperandValue::I64(-34))).unwrap(), "-34".to_string());
        assert_eq!(serde_json::to_string(&Operand::Value(OperandValue::F64(-64.5))).unwrap(), "-64.5".to_string());
        assert_eq!(serde_json::to_string(&Operand::Value(OperandValue::USize(84))).unwrap(), "84".to_string());
        assert_eq!(serde_json::to_string(&Operand::Value(OperandValue::ISize(-79))).unwrap(), "-79".to_string());
        assert_eq!(serde_json::to_string(&Operand::Value(OperandValue::Bool(false))).unwrap(), "false".to_string());
        assert_eq!(serde_json::to_string(&Operand::Value(OperandValue::Str("A lua de Netuno".into()))).unwrap(), r#""A lua de Netuno""#.to_string());
        assert_eq!(serde_json::to_string(&Operand::FieldPath("info.data.value".into())).unwrap(), r#""info.data.value""#.to_string());
    }

    #[test]
    fn serialize_operation() {
        let operation_eq = Operation::Eq(Operand::Value(OperandValue::U64(12)));
        let operation_ne = Operation::Ne(Operand::Value(OperandValue::I64(-34)));
        let operation_gt = Operation::Gt(Operand::Value(OperandValue::F64(-64.5)));
        let operation_ge = Operation::Ge(Operand::Value(OperandValue::USize(84)));
        let operation_lt = Operation::Lt(Operand::Value(OperandValue::ISize(-79)));
        let operation_le = Operation::Le(Operand::Value(OperandValue::Bool(false)));
        let operation_btwn = Operation::Btwn(Operand::Value(OperandValue::Str("Pink moon".into())), Operand::FieldPath("a.b.c".into()));

        assert_eq!(serde_json::to_string(&operation_eq).unwrap(), r#"{"Eq":12}"#.to_string());
        assert_eq!(serde_json::to_string(&operation_ne).unwrap(), r#"{"Ne":-34}"#.to_string());
        assert_eq!(serde_json::to_string(&operation_gt).unwrap(), r#"{"Gt":-64.5}"#.to_string());
        assert_eq!(serde_json::to_string(&operation_ge).unwrap(), r#"{"Ge":84}"#.to_string());
        assert_eq!(serde_json::to_string(&operation_lt).unwrap(), r#"{"Lt":-79}"#.to_string());
        assert_eq!(serde_json::to_string(&operation_le).unwrap(), r#"{"Le":false}"#.to_string());
        assert_eq!(serde_json::to_string(&operation_btwn).unwrap(), r#"{"Btwn":["Pink moon","a.b.c"]}"#.to_string());
    }
}
