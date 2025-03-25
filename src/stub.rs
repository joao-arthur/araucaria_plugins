use std::collections::HashMap;

use araucaria::value::Value;

pub fn bool_stub() -> Value {
    Value::Bool(true)
}

pub fn num_u_stub() -> Value {
    Value::NumU(42)
}

pub fn num_i_stub() -> Value {
    Value::NumI(-42)
}

pub fn num_f_stub() -> Value {
    Value::NumF(-21.5)
}

pub fn str_stub() -> Value {
    Value::Str(String::from("Lorem ipsum"))
}

pub fn arr_bool_stub() -> Value {
    Value::Arr(vec![Value::Bool(false), Value::Bool(true), Value::Bool(false), Value::Bool(true)])
}

pub fn arr_num_u_stub() -> Value {
    Value::Arr(vec![Value::NumU(1), Value::NumU(10), Value::NumU(100)])
}

pub fn arr_num_i_stub() -> Value {
    Value::Arr(vec![Value::NumI(-100), Value::NumI(0), Value::NumI(100)])
}

pub fn arr_num_f_stub() -> Value {
    Value::Arr(vec![Value::NumF(-10.5), Value::NumF(0.5), Value::NumF(10.5)])
}

pub fn arr_num_stub() -> Value {
    Value::Arr(vec![Value::NumU(10), Value::NumI(-10), Value::NumF(1.25)])
}

pub fn arr_str_stub() -> Value {
    Value::Arr(vec![
        Value::Str(String::from("George Harrison")),
        Value::Str(String::from("John Lennon")),
        Value::Str(String::from("Paul McCartney")),
        Value::Str(String::from("Ringo Starr")),
    ])
}

pub fn obj_stub() -> Value {
    Value::Obj(HashMap::from([
        (String::from("name"), Value::Str(String::from("The Beatles"))),
        (
            String::from("members"),
            Value::Arr(vec![
                Value::Str(String::from("George Harrison")),
                Value::Str(String::from("John Lennon")),
                Value::Str(String::from("Paul McCartney")),
                Value::Str(String::from("Ringo Starr")),
            ]),
        ),
        (String::from("start_year"), Value::NumU(1960)),
        (String::from("end_year"), Value::NumU(1960)),
        (String::from("number_of_albums"), Value::NumU(13)),
        (String::from("greatest_band"), Value::Bool(true)),
    ]))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(bool_stub(), Value::Bool(true));
        assert_eq!(num_u_stub(), Value::NumU(42));
        assert_eq!(num_i_stub(), Value::NumI(-42));
        assert_eq!(num_f_stub(), Value::NumF(-21.5));
        assert_eq!(str_stub(), Value::Str(String::from("Lorem ipsum")));
        assert_eq!(
            arr_bool_stub(),
            Value::Arr(vec![
                Value::Bool(false),
                Value::Bool(true),
                Value::Bool(false),
                Value::Bool(true)
            ])
        );
        assert_eq!(
            arr_num_u_stub(),
            Value::Arr(vec![Value::NumU(1), Value::NumU(10), Value::NumU(100)])
        );
        assert_eq!(
            arr_num_i_stub(),
            Value::Arr(vec![Value::NumI(-100), Value::NumI(0), Value::NumI(100)])
        );
        assert_eq!(
            arr_num_f_stub(),
            Value::Arr(vec![Value::NumF(-10.5), Value::NumF(0.5), Value::NumF(10.5)])
        );
        assert_eq!(
            arr_num_stub(),
            Value::Arr(vec![Value::NumU(10), Value::NumI(-10), Value::NumF(1.25)])
        );
        assert_eq!(
            arr_str_stub(),
            Value::Arr(vec![
                Value::Str(String::from("George Harrison")),
                Value::Str(String::from("John Lennon")),
                Value::Str(String::from("Paul McCartney")),
                Value::Str(String::from("Ringo Starr")),
            ])
        );
        assert_eq!(
            obj_stub(),
            Value::Obj(HashMap::from([
                (String::from("name"), Value::Str(String::from("The Beatles"))),
                (
                    String::from("members"),
                    Value::Arr(vec![
                        Value::Str(String::from("George Harrison")),
                        Value::Str(String::from("John Lennon")),
                        Value::Str(String::from("Paul McCartney")),
                        Value::Str(String::from("Ringo Starr")),
                    ]),
                ),
                (String::from("start_year"), Value::NumU(1960)),
                (String::from("end_year"), Value::NumU(1960)),
                (String::from("number_of_albums"), Value::NumU(13)),
                (String::from("greatest_band"), Value::Bool(true)),
            ]))
        );
    }
}
