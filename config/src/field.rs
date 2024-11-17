// Field type enums definations
#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
    String,
    Number,
    Boolean,
    Date,
    Object,
    Array,
}

impl FieldType {

    fn of(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "string" => FieldType::String,
            "number" => FieldType::Number,
            "boolean" => FieldType::Boolean,
            "date" => FieldType::Date,
            "object" => FieldType::Object,
            "array" => FieldType::Array,
            _ => panic!("Invalid field type"),
        }
    }

    fn string(&self) -> &str {
        match self {
            FieldType::String => "string",
            FieldType::Number => "number",
            FieldType::Boolean => "boolean",
            FieldType::Date => "date",
            FieldType::Object => "object",
            FieldType::Array => "array",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub field_type: FieldType,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string() {
        let field_type = FieldType::String;
        assert_eq!(field_type.string(), "string");

        let field_type = FieldType::Number;
        assert_eq!(field_type.string(), "number");

        let field_type = FieldType::Boolean;
        assert_eq!(field_type.string(), "boolean");

        let field_type = FieldType::Date;
        assert_eq!(field_type.string(), "date");

        let field_type = FieldType::Object;
        assert_eq!(field_type.string(), "object");

        let field_type = FieldType::Array;
        assert_eq!(field_type.string(), "array");
    }

    #[test]
    fn test_of() {
        assert_eq!(FieldType::of("String".to_string()), FieldType::String);
        assert_eq!(FieldType::of("Number".to_string()), FieldType::Number);
        assert_eq!(FieldType::of("Boolean".to_string()), FieldType::Boolean);
        assert_eq!(FieldType::of("Date".to_string()), FieldType::Date);
        assert_eq!(FieldType::of("Object".to_string()), FieldType::Object);
        assert_eq!(FieldType::of("Array".to_string()), FieldType::Array);
    }

    #[test]
    #[should_panic(expected = "Invalid field type")]
    fn test_of_panic() {
        FieldType::of("Invalid".to_string());
    }
}