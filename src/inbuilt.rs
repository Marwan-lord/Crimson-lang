use crate::object::Object;

pub fn get_builtin(identifier: &str) -> Option<Object> {
    match identifier {
        "len" => Some(Object::BuiltInFunction(String::from("len"))),
        _ => None,
    }
}

fn process_len(args: &Vec<Object>) -> Object {
    if args.len() != 1 {
        panic!(
            "Expected one argument for len found {} arguments",
            args.len()
        )
    }

    let argument = &args[0];

    match argument {
        Object::String(s) => Object::Integer(s.len() as i64),
        Object::Array(o) => Object::Integer(o.len() as i64),
        _ => panic!("Expected string argument"),
    }
}

pub fn eval_builtin(func_obj: &Object, args: &Vec<Object>) -> Object {
    match func_obj {
        Object::BuiltInFunction(func_name) => match func_name.as_str() {
            "len" => return process_len(args),
            _ => panic!("Invalid inbuilt function"),
        },
        _ => panic!("Expected a function  object but found {}", func_obj),
    }
}
