use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug)]
struct JsonEntry {
    key: String,
    values: Vec<String>,
}

fn process_json(content: &str) -> String {
    let content: String = content
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            !trimmed.is_empty() && !trimmed.starts_with("//")
        })
        .collect::<Vec<_>>()
        .join("\n");

    let content = content.trim();
    let content = if content.starts_with('{') && content.ends_with('}') {
        format!("[{}]", &content[1..content.len() - 1])
    } else {
        content.to_string()
    };

    content
        .lines()
        .map(|line| {
            let trimmed = line.trim();
            if let Some(first_quote) = trimmed.find('"') {
                if let Some(second_quote) = trimmed[first_quote + 1..].find('"') {
                    let key = &trimmed[first_quote + 1..first_quote + 1 + second_quote];
                    if key.chars().all(|c| c.is_numeric()) {
                        if let Some(colon_pos) = trimmed[first_quote + 1 + second_quote..].find(':')
                        {
                            return trimmed[first_quote + 1 + second_quote + colon_pos + 1..]
                                .trim()
                                .to_string();
                        }
                    }
                }
            }
            trimmed.to_string()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn parse_object(json_str: &str, entries: &mut Vec<JsonEntry>) {
    let mut chars = json_str.chars().peekable();
    let mut key = String::with_capacity(32);
    let mut current_values = Vec::with_capacity(8);

    while let Some(c) = chars.next() {
        match c {
            '"' => {
                let mut content = String::with_capacity(32);
                while let Some(&next_c) = chars.peek() {
                    chars.next();
                    if next_c == '"' {
                        break;
                    }
                    content.push(next_c);
                }
                if key.is_empty() && current_values.is_empty() {
                    key = content;
                } else {
                    current_values.push(content);
                }
            }
            '[' => continue,
            ']' => {
                if !key.is_empty() && !current_values.is_empty() {
                    entries.push(JsonEntry {
                        key,
                        values: current_values,
                    });
                    key = String::with_capacity(32);
                    current_values = Vec::with_capacity(8);
                }
            }
            _ => continue,
        }
    }
}

fn process_object(json_str: &str) -> String {
    let mut entries = Vec::with_capacity(8);
    parse_object(json_str, &mut entries);

    let mut result = String::with_capacity(json_str.len());
    result.push_str("{\n");

    let mut merged_entries: Vec<JsonEntry> = Vec::with_capacity(entries.len());
    for entry in entries {
        let mut found = false;
        for existing in merged_entries.iter_mut() {
            if existing.key == entry.key {
                let new_values: Vec<String> = entry
                    .values
                    .iter()
                    .filter(|v| !existing.values.contains(v))
                    .cloned()
                    .collect();
                existing.values.extend(new_values);
                found = true;
                break;
            }
        }
        if !found {
            merged_entries.push(entry);
        }
    }

    for (i, entry) in merged_entries.iter().enumerate() {
        result.push_str(&format!("    \"{}\": [", entry.key));
        for (j, value) in entry.values.iter().enumerate() {
            result.push_str(&format!("\"{}\"", value));
            if j < entry.values.len() - 1 {
                result.push_str(", ");
            }
        }
        result.push(']');
        if i < merged_entries.len() - 1 {
            result.push_str(",\n");
        } else {
            result.push('\n');
        }
    }
    result.push('}');
    result
}

fn count_single_province(city_groups: &HashMap<String, Vec<String>>) -> usize {
    let mut reverse_connections: HashMap<&String, Vec<&String>> = HashMap::new();
    for (city, connections) in city_groups {
        for connected_city in connections {
            reverse_connections
                .entry(connected_city)
                .or_insert_with(Vec::new)
                .push(city);
        }
    }

    let mut provinces = 0;
    let mut processed_cities: HashSet<&String> = HashSet::new();

    for city in city_groups.keys() {
        if processed_cities.contains(city) {
            continue;
        }

        provinces += 1;
        let mut cities_to_process = vec![city];

        while let Some(current_city) = cities_to_process.pop() {
            if !processed_cities.insert(current_city) {
                continue;
            }

            if let Some(connections) = city_groups.get(current_city) {
                for connected_city in connections {
                    if !processed_cities.contains(connected_city) {
                        cities_to_process.push(connected_city);
                    }
                }
            }

            if let Some(reverse_deps) = reverse_connections.get(current_city) {
                for &other_city in reverse_deps {
                    if !processed_cities.contains(other_city) {
                        cities_to_process.push(other_city);
                    }
                }
            }
        }
    }

    provinces
}

pub fn count_provinces() -> &'static str {
    let content = fs::read_to_string("district.json").expect("Failed to read file");
    let processed_content = process_json(&content);

    let mut current_object = String::with_capacity(1024);
    let mut depth = 0;
    let mut all_objects = Vec::new();

    for c in processed_content.chars() {
        match c {
            '{' => {
                depth += 1;
                if depth == 1 {
                    current_object.clear();
                }
                current_object.push(c);
            }
            '}' => {
                depth -= 1;
                current_object.push(c);
                if depth == 0 {
                    let processed_object = process_object(&current_object);
                    all_objects.push(processed_object);
                }
            }
            _ => {
                if depth > 0 {
                    current_object.push(c);
                }
            }
        }
    }

    let provinces: Vec<String> = all_objects
        .iter()
        .map(|object_str| {
            let city_group: HashMap<String, Vec<String>> =
                serde_json::from_str(object_str).expect("Failed to parse JSON object");
            count_single_province(&city_group).to_string()
        })
        .collect();

    let result = provinces.join(",");
    Box::leak(result.into_boxed_str())
}
