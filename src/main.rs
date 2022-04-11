use std::env::{args, Args};

fn main() {
    let mut args:Args = args();

    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    let first_number: f32 = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();

    let result = operate(operator, first_number, second_number);
    println!("using if else : {:?}",output(first_number,operator,second_number,result));

    let result1 = operate1(operator, first_number, second_number);
    println!("using match   : {:?}",output1(first_number,operator,second_number,result1));
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32{
    if operator == '+'{
        return first_number + second_number;
    }else if operator == '-'{
        return first_number - second_number;
    }else if operator == '/'{
        return first_number / second_number;
    }else if operator == '*'{
        return first_number * second_number
    }else{
        return 0.0;
    }
}

fn output(first_number:f32, operator:char, second_number:f32, result:f32) -> String{
    format!("{} {} {} = {}", first_number, operator, second_number, result)
}

fn operate1(operator: char, first_number: f32, second_number: f32) -> f32{
    match operator{                                                                             // Using match instead of if else
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '/' => first_number / second_number,
        '*' => first_number * second_number,
        _ => panic!("Invalid operator used")
    }
}

fn output1(first_number:f32, operator:char, second_number:f32, result:f32) -> String{
    format!("{} {} {} = {}", first_number, operator, second_number, result)
}