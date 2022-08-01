pub mod error;
pub mod executor;
pub mod frame;

#[cfg(test)]
mod test {
    use crate::ast::test::get_test_program;
    use crate::runtime::executor::execute_program;
    use crate::runtime::frame::Frame;

    #[test]
    pub fn test_simple_calculator() {
        let program = get_test_program("simple_calculator.lrlang");
        let mut root_frame = Frame::default();
        execute_program(&mut root_frame, &program).unwrap();

        assert_eq!(root_frame.variable_value("my_variable").unwrap(), 5);
        assert_eq!(root_frame.variable_value("my_variable_1").unwrap(), 5);
        assert_eq!(root_frame.variable_value("sum").unwrap(), 10);
        assert_eq!(root_frame.variable_value("mult").unwrap(), 1200);
        assert_eq!(root_frame.variable_value("div").unwrap(), 60);
        assert_eq!(root_frame.variable_value("expression").unwrap(), 125);
    }
}
