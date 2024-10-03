mod error;
pub mod lexer;
pub mod node;
mod operator;
pub mod parser;
mod token;
mod value;

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::value::Value;
    use std::collections::HashMap;

    fn parse(input_string: &str, context: &HashMap<&str, f64>) -> Value {
        let tokens = Lexer::tokenize(input_string).unwrap();
        let ast_node = Parser::new(tokens).parse().unwrap();

        ast_node.evaluate(&context).unwrap()
    }

    #[test]
    fn eval_comparison_expression() {
        let input_string = "-6 < 3";

        let context: HashMap<&str, f64> = HashMap::new();
        let result = parse(input_string, &context);

        assert_eq!(result, Value::Bool(true));

        let input_string = "6 < 3";

        let context: HashMap<&str, f64> = HashMap::new();
        let result = parse(input_string, &context);

        assert_eq!(result, Value::Bool(false));

        let input_string = "6 == 5";

        let context: HashMap<&str, f64> = HashMap::new();
        let result = parse(input_string, &context);

        assert_eq!(result, Value::Bool(false));

        let input_string = "6 == 6";

        let context: HashMap<&str, f64> = HashMap::new();
        let result = parse(input_string, &context);

        assert_eq!(result, Value::Bool(true));
    }

    #[test]
    fn eval_and_logical_expression() {
        let input_string = "6 == 5 and 3 == 4"; // 0 & 0 = 0

        let context: HashMap<&str, f64> = HashMap::new();
        let result = parse(input_string, &context);

        assert_eq!(result, Value::Bool(false));

        let input_string = "6 == 5 and 3 == 3"; // 0 & 1 = 0

        let context: HashMap<&str, f64> = HashMap::new();
        let result = parse(input_string, &context);

        assert_eq!(result, Value::Bool(false));

        let input_string = "6 == 6 and 3 == 4"; // 1 & 0 = 0

        let context: HashMap<&str, f64> = HashMap::new();
        let result = parse(input_string, &context);

        assert_eq!(result, Value::Bool(false));

        let input_string = "6 == 6 and 4 == 4"; // 1 & 1 = 1

        let context: HashMap<&str, f64> = HashMap::new();
        let result = parse(input_string, &context);

        assert_eq!(result, Value::Bool(true));
    }

    #[test]
    fn eval_or_logical_expression() {
        let input_string = "6 == 5 or 3 == 4"; // 0 | 0 = 0

        let context: HashMap<&str, f64> = HashMap::new();
        let result = parse(input_string, &context);

        assert_eq!(result, Value::Bool(false));

        let input_string = "6 == 5 or 3 == 3"; // 0 | 1 = 1

        let context: HashMap<&str, f64> = HashMap::new();
        let result = parse(input_string, &context);

        assert_eq!(result, Value::Bool(true));

        let input_string = "6 == 6 or 3 == 4"; // 1 | 0 = 1

        let context: HashMap<&str, f64> = HashMap::new();
        let result = parse(input_string, &context);

        assert_eq!(result, Value::Bool(true));

        let input_string = "6 == 6 or 4 == 4"; // 1 | 1 = 1

        let context: HashMap<&str, f64> = HashMap::new();
        let result = parse(input_string, &context);

        assert_eq!(result, Value::Bool(true));
    }

    #[test]
    fn eval_arithmetic_expression() {
        // Test simple addition
        let input_string = "6 + 4";

        let context: HashMap<&str, f64> = HashMap::new();
        let result = parse(input_string, &context);

        assert_eq!(result, Value::Float(10.0));

        // Test simple subtraction
        let input_string = "6 - 4";

        let context: HashMap<&str, f64> = HashMap::new();
        let result = parse(input_string, &context);

        assert_eq!(result, Value::Float(2.0));

        // Test simple subtraction with negative result
        let input_string = "4 - 7";

        let context: HashMap<&str, f64> = HashMap::new();
        let result = parse(input_string, &context);

        assert_eq!(result, Value::Float(-3.0));

        // Test simple multiplication
        let input_string = "7 * 3";

        let context: HashMap<&str, f64> = HashMap::new();
        let result = parse(input_string, &context);

        assert_eq!(result, Value::Float(21.0));

        // Test simple division
        let input_string = "10 / 2";

        let context: HashMap<&str, f64> = HashMap::new();
        let result = parse(input_string, &context);

        assert_eq!(result, Value::Float(5.0));

        // Test arithmetic priority
        let input_string = "10 - 3 * 3";

        let context: HashMap<&str, f64> = HashMap::new();
        let result = parse(input_string, &context);

        assert_eq!(result, Value::Float(1.0));

        // Test parentheses in arithmetic expression
        let input_string = "(10 - 3) * 3";

        let context: HashMap<&str, f64> = HashMap::new();
        let result = parse(input_string, &context);

        assert_eq!(result, Value::Float(21.0));

        // Test arithmetic addition with negative value
        let input_string = "5 + -3";

        let context: HashMap<&str, f64> = HashMap::new();
        let result = parse(input_string, &context);

        assert_eq!(result, Value::Float(2.0));

        // Test arithmetic subtraction with negative value
        let input_string = "5 -- 3";

        let context: HashMap<&str, f64> = HashMap::new();
        let result = parse(input_string, &context);

        assert_eq!(result, Value::Float(8.0));

        // Test complex arithmetic expression
        let input_string = "3 + 5 * (2 - 8) / 4 - 7";

        let context: HashMap<&str, f64> = HashMap::new();
        let result = parse(input_string, &context);

        assert_eq!(result, Value::Float(-11.5));
    }

    #[test]
    fn eval_arithmetic_expression_with_variables() {
        // Test simple division
        let input_string = "x + (y - 10) / x + 4 * y";

        let mut context: HashMap<&str, f64> = HashMap::new();
        context.insert("x", 2.0);
        context.insert("y", 16.0);

        let result = parse(input_string, &context);

        assert_eq!(result, Value::Float(69.0));
    }
}
