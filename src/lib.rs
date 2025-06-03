#![allow(dead_code)]
use std::env::current_dir;
use std::fs::{create_dir, File, OpenOptions};
use std::io::Write;
use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub enum SensorValue {
    F32(f32),
    U8(u8),
    U16(u16),
    U32(u32),
}

#[derive(Debug, Clone)]
pub enum DataType {
    F32,
    U8,
    U16,
    U32,
}

#[derive(Debug, Clone)]
pub struct Value {
    pub var_name: String,
    pub type_: DataType,
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
        .position(|&b| b == b'\\')
        .ok_or("No end marker '\\' found")?;

    Ok((start, end))
}

pub fn get_sensor(buffer: Vec<u8>) -> Result<Vec<Sensor>, String> {
    let (start, end) = get_indices(&buffer)?;
    let properties_as_binary = &buffer[start..=end];

    let properties = String::from_utf8(properties_as_binary.to_vec())
        .map_err(|e| format!("Invalid UTF-8 sequence: {}", e))?;

    parse_sensor_properties(&properties)
}

pub fn parse_sensor_properties(config_str: &str) -> Result<Vec<Sensor>, String> {
    let mut sensors = Vec::new();

    if !config_str.starts_with('$') || !config_str.ends_with('\\') {
        return Err("Invalid format. Must start with $ and end with \\.".to_string());
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
                    let data_type = match parts[1].to_lowercase().as_str() {
                        "f32" => DataType::F32,
                        "u8" => DataType::U8,
                        "u16" => DataType::U16,
                        "u32" => DataType::U32,
                        _ => return Err(format!("Unsupported data type: '{}'", parts[1])),
                    };
                    let value = Value {
                        var_name: parts[0].to_string(),
                        type_: data_type,
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
    if data.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::new();
    let mut chars = data.chars();

    while let Some(_) = chars.clone().next() {
        let chunk: String = chars.by_ref().take(8).collect();
        if !chunk.is_empty() {
            result.push(chunk);
        }
    }

    result
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

pub fn id_to_hex(id: &String) -> Result<String, String> {
    let num: u32 = id
        .parse()
        .map_err(|e| format!("Failed to parse '{}' as u32: {}", id, e))?;
    let bytes = num.to_le_bytes();
    let hex_str = bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();
    Ok(hex_str)
}

pub fn directory_generator() -> Result<String, Box<dyn std::error::Error>> {
    let working_directory = current_dir()?;

    let mut count = 0;

    if cfg!(windows) {
        let fullpath = format!("{}\\LOG{}", working_directory.display(), count);
        //println!("{fullpath}");
        while !Path::new(&fullpath).exists() {
            let fullpath = format!("{}\\LOG{}", working_directory.display(), count);
            //println!("zap");
            let _ = create_dir(fullpath.clone());
            count += 1;
        }
        Ok(fullpath)
    } else {
        let mut fullpath = format!("{}/LOG{}", working_directory.display(), count);
        //println!("{fullpath}");
        while Path::new(&fullpath).exists() {
            count += 1;
            fullpath = format!("{}/LOG{}", working_directory.display(), count);
        }
        let _ = create_dir(fullpath.clone());
        Ok(fullpath)
    }
}
