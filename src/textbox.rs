use crate::PropertyType;

pub fn text_to_type(text: String, text_type: PropertyType) -> String {
    match text_type {
        PropertyType::Bool => {
            if text != "true".to_owned() || text != "false".to_owned() {
                return "false".to_owned();
            }

            text
        },
        PropertyType::Number => {
            let mut text_copy = String::new();

            for c in text.clone().chars() {
                if c.is_numeric() {
                    text_copy.push(c);
                }

                if c == '.' {
                    text_copy.push(c);
                }
            }

            text_copy
        },
        PropertyType::Custom(_) => {
            // Unique arm because may change in future
            text
        },
        _ => {
            text
        }
    }
}