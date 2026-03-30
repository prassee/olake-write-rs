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

fn main() {
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
}
