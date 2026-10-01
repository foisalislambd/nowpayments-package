//! Query parameter helpers for list/filter endpoints

use serde_json::Value;

/// Expand `id` / `status` filter values into repeated query keys (`id=1&id=2`).
pub fn push_json_filter_query(out: &mut Vec<(&'static str, String)>, key: &'static str, value: &Value) {
    match value {
        Value::Array(arr) => {
            for item in arr {
                push_scalar_query(out, key, item);
            }
        }
        other => push_scalar_query(out, key, other),
    }
}

fn push_scalar_query(out: &mut Vec<(&'static str, String)>, key: &'static str, value: &Value) {
    let s = match value {
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.clone(),
        Value::Bool(b) => b.to_string(),
        _ => return,
    };
    out.push((key, s));
}
