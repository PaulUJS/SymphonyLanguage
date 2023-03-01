use std::any::type_name;

#[derive(Debug)]
enum Types {
    UI8 {r#type: String, value: u8},
    UI16 {r#type: String, value: u16},
    UI32 {r#type: String, value: u32},
    UI64 {r#type: String, value: u64},
    UI128 {r#type: String, value: u128},
}

#[derive(Debug)]
struct Program {
    r#type: String,
    body: Types,
}

pub fn parse_data(input: &String) {
    let byte = input.as_bytes();
    println!("{:?}", byte);
    let input: u128 = input.parse().unwrap();

    let return_val = Types::UI128{ 
        r#type: String::from("UI128"), 
        value: input,
    };
    program(return_val);
}

fn program(body: Types) {
    let prog = Program{
        r#type: String::from("Program"), 
        body: body
    };
    println!("{:#?}", prog);
}