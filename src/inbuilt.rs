use crate::object::Object;

pub fn get_builtin(identifier: &str) -> Option<Object> {
    match identifier {
        "len" => Some(Object::BuiltInFunction(String::from("len"))),
        "print" => Some(Object::BuiltInFunction(String::from("print"))),
        _ => None,
    }
}

fn process_len(args: &[Object]) -> Object {
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
        _ => panic!("Expected string or array argument"),
    }
}

fn procces_print(args: &[Object]) -> Object {
    if args.len() != 1 {
        panic!(
            "Expected one argument for print found {} arguments",
            args.len()
        )
    }

    let argument = &args[0];

    match argument {
        Object::String(s) => Object::String(s.to_string()),
        Object::Array(a) => Object::Array(a.to_vec()),
        Object::HashMap(h) => Object::HashMap(h.clone()),
        Object::Integer(i) => Object::Integer(*i),
        _ => panic!("Expected a string | array | hashmap | integer  argument")
    }
}

pub fn eval_builtin(func_obj: &Object, args: &[Object]) -> Object {
    match func_obj {
        Object::BuiltInFunction(func_name) => match func_name.as_str() {
            "len" => process_len(args),
            "print" => procces_print(args),
            _ => panic!("Invalid inbuilt function"),
        },
        _ => panic!("Expected a function object but found {}", func_obj),
    }
}
