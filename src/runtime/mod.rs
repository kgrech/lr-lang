pub mod error;
pub mod executor;
pub mod frame;
pub mod operations;

#[cfg(test)]
mod test {
    use crate::ast::test::get_test_program;
    use crate::ast::value::Value;
    use crate::runtime::executor::execute_program;
    use crate::runtime::frame::Frame;
    use rstest::*;

    #[test]
    pub fn test_simple_calculator() {
        let program = get_test_program("valid", "simple_calculator.lrlang");
        let mut root_frame = Frame::default();
        execute_program(&mut root_frame, &program).unwrap();

        assert_eq!(
            root_frame.variable_value("my_variable").unwrap(),
            Value::Int(5)
        );
        assert_eq!(
            root_frame.variable_value("my_variable_1").unwrap(),
            Value::Int(5)
        );
        assert_eq!(root_frame.variable_value("sum").unwrap(), Value::Int(10));
        assert_eq!(root_frame.variable_value("mult").unwrap(), Value::Int(1200));
        assert_eq!(root_frame.variable_value("div").unwrap(), Value::Int(60));
        assert_eq!(
            root_frame.variable_value("expression").unwrap(),
            Value::Int(125)
        );
        assert_eq!(
            root_frame.variable_value("float").unwrap(),
            Value::Float(5.5)
        );
        assert_eq!(
            root_frame.variable_value("float_plus_float").unwrap(),
            Value::Float(11.0)
        );
        assert_eq!(
            root_frame.variable_value("float_plus_int").unwrap(),
            Value::Float(10.5)
        );
        assert_eq!(
            root_frame.variable_value("int_plus_float").unwrap(),
            Value::Float(10.5)
        );
        assert_eq!(
            root_frame.variable_value("float_minus_float").unwrap(),
            Value::Float(0.5)
        );
        assert_eq!(
            root_frame.variable_value("float_minus_int").unwrap(),
            Value::Float(0.5)
        );
        assert_eq!(
            root_frame.variable_value("int_minus_float").unwrap(),
            Value::Float(-0.5)
        );
        assert_eq!(
            root_frame.variable_value("float_times_float").unwrap(),
            Value::Float(11.0)
        );
        assert_eq!(
            root_frame.variable_value("float_times_int").unwrap(),
            Value::Float(11.0)
        );
        assert_eq!(
            root_frame.variable_value("int_times_float").unwrap(),
            Value::Float(11.0)
        );
        assert_eq!(
            root_frame.variable_value("float_div_float").unwrap(),
            Value::Float(1.1)
        );
        assert_eq!(
            root_frame.variable_value("float_div_int").unwrap(),
            Value::Float(1.1)
        );
        assert_eq!(
            root_frame.variable_value("int_div_float").unwrap(),
            Value::Float(1.1)
        );
    }

    #[test]
    pub fn test_string_operations() {
        let program = get_test_program("valid", "string_operations.lrlang");
        let mut root_frame = Frame::default();
        execute_program(&mut root_frame, &program).unwrap();

        assert_eq!(
            root_frame.variable_value("string").unwrap(),
            Value::String("100".to_owned())
        );
        assert_eq!(
            root_frame.variable_value("string_concat_1").unwrap(),
            Value::String("abc100".to_owned())
        );
        assert_eq!(
            root_frame.variable_value("string_concat_2").unwrap(),
            Value::String("100100".to_owned())
        );
        assert_eq!(
            root_frame.variable_value("string_concat_3").unwrap(),
            Value::String("100 abc".to_owned())
        );
        assert_eq!(
            root_frame.variable_value("string_concat_4").unwrap(),
            Value::String("abcefg".to_owned())
        );
        assert_eq!(
            root_frame.variable_value("string_plus_int").unwrap(),
            Value::String("int=5".to_owned())
        );
        assert_eq!(
            root_frame.variable_value("string_plus_float").unwrap(),
            Value::String("float=5.5".to_owned())
        );
        assert_eq!(
            root_frame.variable_value("string_plus_int_var").unwrap(),
            Value::String("int=5".to_owned())
        );
        assert_eq!(
            root_frame.variable_value("string_plus_float_var").unwrap(),
            Value::String("float=5.5".to_owned())
        );
        assert_eq!(
            root_frame.variable_value("redefine").unwrap(),
            Value::String("new_value".to_owned())
        );
    }
    #[test]
    pub fn test_circle_square() {
        let program = get_test_program("valid", "circle_square.lrlang");
        let mut root_frame = Frame::default();
        execute_program(&mut root_frame, &program).unwrap();

        assert_eq!(
            root_frame.variable_value("hello_world").unwrap(),
            Value::String("Hello World".to_owned())
        );
        assert_eq!(
            root_frame.variable_value("value").unwrap(),
            Value::String("The square of the circle with the r = 5 is 157".to_owned())
        );
    }

    #[rstest]
    #[case(
        "string_mult_num",
        "Unable to evalutate expression (string * num): Operation String * Int is not defined"
    )]
    #[case(
        "string_div_num",
        "Unable to evalutate expression (\"100\" / 5): Operation String / Int is not defined"
    )]
    #[case(
        "string_minus_num",
        "Unable to evalutate expression (string - num): Operation String - Int is not defined"
    )]
    #[case(
        "float_minus_string",
        "Unable to evalutate expression (num - string): Operation Float - String is not defined"
    )]
    #[case(
        "assign_int_to_string",
        "Unable to assign the value of type Int to variable 'string' of type String"
    )]
    pub fn test_runtime_error(#[case] file: &str, #[case] expected_error: &str) {
        let program = get_test_program("runtime_error", &format!("{}.lrlang", file));
        let mut root_frame = Frame::default();
        let result = execute_program(&mut root_frame, &program);
        assert!(result.is_err());
        let error = result.err().unwrap().to_string();
        assert_eq!(expected_error, error);
    }
}
