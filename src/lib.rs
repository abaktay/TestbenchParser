#![allow(dead_code)]
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Value {
    pub var_name: String,
    pub type_: String,
    pub unit: String,
}

#[derive(Debug, Clone)]
pub struct Sensor {
    pub name: String,
    pub values: Vec<Value>,
    pub id: String,
}

pub fn get_indices(buffer: &[u8]) -> Result<(usize, usize), String> {
    let start = buffer
        .iter()
        .position(|&b| b == b'$')
        .ok_or("No start marker '$' found")?;
    let end = buffer
        .iter()
        .position(|&b| b == b'&')
        .ok_or("No end marker '&' found")?;

    Ok((start, end))
}

pub fn get_sensor(buffer: Vec<u8>) -> Result<Vec<Sensor>, String> {
    let (start, end) = get_indices(&buffer)?;

    // let f = buffer.iter().filter(|&b| *b == 31).count();
    // println!("LAAAN {:?}", f);
    let properties_as_binary = &buffer[start..=end];

    let properties = String::from_utf8(properties_as_binary.to_vec())
        .map_err(|e| format!("Invalid UTF-8 sequence: {}", e))?;

    let mut f = File::create("zrt.bin").map_err(|e| e.to_string())?;
    f.write_all(&buffer[end..]).map_err(|e| e.to_string())?;
    parse_sensor_properties(&properties)
}

pub fn parse_sensor_properties(config_str: &str) -> Result<Vec<Sensor>, String> {
    let mut sensors = Vec::new();

    if !config_str.starts_with('$') || !config_str.ends_with('&') {
        return Err("Invalid format. Must start with $ and end with &.".to_string());
    }

    // Removes $ and & from the string
    let content = &config_str[1..config_str.len() - 1];

    let sections: Vec<&str> = content
        .split('|')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    for section in sections {
        let tokens: Vec<&str> = section.split_whitespace().collect();
        if tokens.is_empty() {
            return Err("Empty sensor section.".to_string());
        }

        let name = tokens[0].to_string();
        let mut values = Vec::new();
        let mut id = None;

        for token in &tokens[1..] {
            if !token.starts_with('<') || !token.ends_with('>') || token.len() > 128 {
                return Err(format!("Invalid token format: '{}'", token));
            }

            let entry_content = &token[1..token.len() - 1];
            let parts: Vec<&str> = entry_content.split(',').map(|s| s.trim()).collect();

            match parts.len() {
                1 => {
                    if id.is_some() {
                        return Err("Multiple IDs found for a sensor.".to_string());
                    }
                    id = Some(parts[0].to_string());
                }
                3 => {
                    let value = Value {
                        var_name: parts[0].to_string(),
                        type_: parts[1].to_string(),
                        unit: parts[2].to_string(),
                    };
                    values.push(value);
                }
                _ => {
                    return Err(format!("Invalid entry format: '{}'", entry_content));
                }
            }
        }

        let id = id.ok_or("Sensor ID is missing.".to_string())?;
        sensors.push(Sensor { name, values, id });
    }

    Ok(sensors)
}

pub fn divide_data(data: &str) -> Vec<String> {
    if data.len() > 0 {
        let mut chars = data.chars();
        let sub_strings = (0..)
            .map(|_| chars.by_ref().take(6).collect::<String>())
            .take_while(|s| !s.is_empty())
            .collect::<Vec<_>>();

        // println!("{sub_strings:?}");
        sub_strings
    } else {
        Vec::new()
    }
}

pub fn csv_starter(sensor: &Sensor, file_path: &Path) -> Result<File, String> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)
        .map_err(|e| e.to_string())?;

    let header = sensor
        .values
        .iter()
        .map(|val| format!("{} ({})", val.var_name, val.unit))
        .collect::<Vec<_>>()
        .join(",");

    writeln!(file, "{}", header).map_err(|e| e.to_string())?;

    Ok(file)
}

pub fn id_to_hex(id: i32) -> String {
    format!("{:0<6x}", id) //.chars().rev().collect::<String>()
}
