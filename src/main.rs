enum PropertyError {
    EmptyKey,
    ValueTooLong(String),
}

fn validate_property(key: &str, value: &str) -> Result<(), PropertyError> {
    if key.is_empty() {
        return Err(PropertyError::EmptyKey);
    }
    if value.len() > 10 {
        return Err(PropertyError::ValueTooLong(value.to_string()));
    }
    Ok(())
}

fn get_len(value: &str) -> usize {
    value.len()
}

fn append(a: String) -> String {
    a + " world"
}

fn parse_int(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse::<i32>()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let mut properties = std::collections::HashMap::new();
    properties.insert("username", "admin");
    properties.insert("public_key", "public_erypted_key");

    for (k, v) in &properties {
        match validate_property(k, v) {
            Ok(_) => println!("Property '{}' is valid.", k),
            Err(e) => match e {
                PropertyError::EmptyKey => println!("Error: Key cannot be empty."),
                PropertyError::ValueTooLong(val) => println!("Error: Value '{}' is too long.", val),
            },
        }
    }

    println!("Length of 'username' value: {}", get_len("ping"));
    println!("Appended string: {}", append("Hello".to_string()));
    match validate_property("", "value_and values") {
        Ok(_) => println!("Property is valid."),
        Err(e) => match e {
            PropertyError::EmptyKey => println!("Error: Key cannot be empty."),
            PropertyError::ValueTooLong(val) => println!("Error: Value '{}' is too long.", val),
        },
    };
    match validate_property("key", "value_and_values") {
        Ok(_) => println!("Property is valid."),
        Err(e) => match e {
            PropertyError::EmptyKey => println!("Error: Key cannot be empty."),
            PropertyError::ValueTooLong(val) => println!("Error: Value '{}' is too long.", val),
        },
    };
    let sum = parse_int("234")? + parse_int("23")?;
    println!("Sum: {}", sum);
    Ok(())
}
