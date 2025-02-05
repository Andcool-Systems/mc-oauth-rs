use serde_json::{json, Value};
use std::collections::HashMap;

#[allow(dead_code)]
/// JSON text format parser (unused)
pub fn format_motd(motd: &str) -> Value {
    let color_map = HashMap::from([
        ('0', "black"), ('1', "dark_blue"), ('2', "dark_green"), ('3', "dark_aqua"),
        ('4', "dark_red"), ('5', "dark_purple"), ('6', "gold"), ('7', "gray"),
        ('8', "dark_gray"), ('9', "blue"), ('a', "green"), ('b', "aqua"),
        ('c', "red"), ('d', "light_purple"), ('e', "yellow"), ('f', "white")
    ]);
    let style_map = HashMap::from([
        ('k', "obfuscated"), ('l', "bold"), ('m', "strikethrough"),
        ('n', "underlined"), ('o', "italic"), ('r', "reset")
    ]);

    let mut json_array = Vec::new();
    let mut current_text = String::new();
    let mut current_json = json!({});

    let chars: Vec<char> = motd.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '§' && i + 1 < chars.len() {
            if !current_text.is_empty() {
                current_json["text"] = Value::String(current_text.clone());
                json_array.push(current_json);
                current_text.clear();
                current_json = json!({});
            }
            
            let format_char = chars[i + 1];
            if let Some(color) = color_map.get(&format_char) {
                current_json["color"] = Value::String(color.to_string());
            } else if let Some(style) = style_map.get(&format_char) {
                current_json[style] = Value::Bool(true);
            }
            
            i += 2;
        } else {
            current_text.push(chars[i]);
            i += 1;
        }
    }

    if !current_text.is_empty() {
        current_json["text"] = Value::String(current_text);
        json_array.push(current_json);
    }

    Value::Array(json_array)
}
