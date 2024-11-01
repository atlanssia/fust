// Field type enums definations
pub enum FieldType {
    String,
    Number,
    Boolean,
    Date,
    Object,
    Array,
}

impl FieldType {
    fn name(&self) -> &str {
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

pub fn to_field_type(value: String) -> FieldType {
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