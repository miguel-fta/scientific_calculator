use regex::Regex;

fn main() {
    //Regex
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let re_sus = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();
    let re_mult = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();
    let re_div = Regex::new(r"(\d+)\s?/\s?(\d+)").unwrap();

    //Traer Datos Usuarios
    let mut expression: String = String::new();
    println!("Introduce tu expresion: ");
    std::io::stdin().read_line(&mut expression).unwrap();

    expression = make_operation(re_mult, expression, "*");
    expression = make_operation(re_div, expression, "/");
    expression = make_operation(re_add, expression, "+");
    expression = make_operation(re_sus, expression, "-");

    println!("Resultado: {}", expression);
}

fn make_operation(reg: Regex, mut expression: String, operation: &str) -> String {
    loop {
        let caps = reg.captures(expression.as_str());

        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        let result = match operation {
            "+" => left_value + right_value,
            "-" => left_value - right_value,
            "*" => left_value * right_value,
            "/" => left_value / right_value,
            _ => 0,
        };

        expression = expression.replace(cap_expression, &result.to_string())
    }

    return expression;
}
