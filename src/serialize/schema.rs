use serde::{Serialize, Serializer};

#[derive(Debug, PartialEq, Clone)]
pub enum EnumValues {
    USize(Vec<usize>),
    ISize(Vec<isize>),
    Str(Vec<String>),
}

impl Serialize for EnumValues {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            EnumValues::USize(value) => value.serialize(serializer),
            EnumValues::ISize(value) => value.serialize(serializer),
            EnumValues::Str(value) => value.serialize(serializer),
        }
    }
}

pub fn to_enum_values(enum_values: araucaria::schema::EnumValues) -> EnumValues {
    match enum_values {
        araucaria::schema::EnumValues::USize(values) => EnumValues::USize(values),
        araucaria::schema::EnumValues::ISize(values) => EnumValues::ISize(values),
        araucaria::schema::EnumValues::Str(values) => EnumValues::Str(values),
    }
}

#[cfg(test)]
mod tests {
    use super::{EnumValues, to_enum_values};

    #[test]
    fn araucaria_enum_values_to_enum_values() {
        let araucaria_enum_usize = araucaria::schema::EnumValues::USize(vec![0, 3, 6, 9, 12, 15, 18]);
        let araucaria_enum_isize = araucaria::schema::EnumValues::ISize(vec![0, -3, 6, -9, 12, -15]);
        let araucaria_enum_str = araucaria::schema::EnumValues::Str(vec!["PEDRA".into(), "PAPEL".into(), "TESOURA".into()]);

        assert_eq!(to_enum_values(araucaria_enum_usize), EnumValues::USize(vec![0, 3, 6, 9, 12, 15, 18]));
        assert_eq!(to_enum_values(araucaria_enum_isize), EnumValues::ISize(vec![0, -3, 6, -9, 12, -15]));
        assert_eq!(to_enum_values(araucaria_enum_str), EnumValues::Str(vec!["PEDRA".into(), "PAPEL".into(), "TESOURA".into()]));
    }

    #[test]
    fn serialize_enum_values() {
        let str_values = vec!["PEDRA".into(), "PAPEL".into(), "TESOURA".into()];
        assert_eq!(serde_json::to_string(&EnumValues::USize(vec![0, 3, 6, 9, 12, 15, 18])).unwrap(), r#"[0,3,6,9,12,15,18]"#.to_string());
        assert_eq!(serde_json::to_string(&EnumValues::ISize(vec![0, -3, 6, -9, 12, -15])).unwrap(), r#"[0,-3,6,-9,12,-15]"#.to_string());
        assert_eq!(serde_json::to_string(&EnumValues::Str(str_values)).unwrap(), r#"["PEDRA","PAPEL","TESOURA"]"#.to_string());
    }
}
