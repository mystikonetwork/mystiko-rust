use serde::Serialize;
use serde_json::Value;

pub const SENSITIVE_KEYWORDS: [&str; 4] = ["secret", "key", "password", "credential"];

pub fn to_safe_json_string<T>(object: &T, pretty: bool) -> Result<String, serde_json::Error>
where
    T: Serialize,
{
    to_safe_json_string_with_keywords::<T, &str>(object, pretty, &SENSITIVE_KEYWORDS)
}

pub fn to_safe_json_string_with_keywords<T, S>(
    object: &T,
    pretty: bool,
    sensitive_keywords: &[S],
) -> Result<String, serde_json::Error>
where
    T: Serialize,
    S: AsRef<str>,
{
    let json_value = serde_json::to_value(object)?;
    let safe_json_value = replace_sensitive_value(
        json_value,
        &sensitive_keywords
            .iter()
            .map(|keyword| keyword.as_ref())
            .collect::<Vec<_>>(),
    );
    if pretty {
        serde_json::to_string_pretty(&safe_json_value)
    } else {
        serde_json::to_string(&safe_json_value)
    }
}

fn replace_sensitive_value(json_value: Value, sensitive_keywords: &[&str]) -> Value {
    match json_value {
        Value::Array(values) => Value::Array(
            values
                .into_iter()
                .map(|value| replace_sensitive_value(value, sensitive_keywords))
                .collect::<Vec<_>>(),
        ),
        Value::Object(object) => Value::Object(
            object
                .into_iter()
                .map(|(key, value)| match value {
                    Value::String(value_str) => {
                        if is_sensitive_field(&key, sensitive_keywords) {
                            (key, Value::String(replace_sensitive_string(&value_str)))
                        } else {
                            (key, Value::String(value_str))
                        }
                    }
                    _ => (key, replace_sensitive_value(value, sensitive_keywords)),
                })
                .collect(),
        ),
        _ => json_value,
    }
}

fn replace_sensitive_string(string_value: &str) -> String {
    let mask = "****".to_string();
    if string_value.len() > 2 {
        format!(
            "{}{}{}",
            string_value.chars().next().unwrap(),
            mask,
            string_value.chars().last().unwrap()
        )
    } else {
        mask
    }
}

fn is_sensitive_field(field_name: &str, sensitive_keywords: &[&str]) -> bool {
    sensitive_keywords
        .iter()
        .any(|keyword| field_name.to_ascii_lowercase().contains(&keyword.to_ascii_lowercase()))
}
