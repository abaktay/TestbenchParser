#![allow(dead_code)]
use parser::*;
use std::fs::File;
use std::io::{Read, Write};
use std::env;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let signal = "1c1c1c1c";
    //let signal = "ff000000".repeat(4);

    let directory = directory_generator().unwrap();
    let file_path: Vec<String> = env::args().collect();

    let mut sensor_list: Vec<Sensor> = Vec::new();
    let mut file = File::open(file_path[1].clone())?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    match get_sensor(buffer.clone()) {
        Ok(sensors) => {
            for sensor in sensors {
                sensor_list.push(sensor.clone());
                // println!("Sensor: {} (ID: {})", sensor.name, sensor.id);
                // for value in sensor.values {
                //     println!(
                //         "  Value: {}, Type: {}, Unit: {}",
                //         value.var_name, value.type_, value.unit
                // }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    let Ok((_, end_idx)) = get_indices(&buffer) else {
        panic!("Error at get_indices()");
    };

    let data = &buffer[end_idx+1..];


    // turns binary to string in hexadecimal format
    let hex_string: String = data.iter().map(|byte| format!("{:02x}", byte)).collect();

    
    // splits data with the end-of-data signal as a vector
    let data_split: Vec<&str> = hex_string.split(&signal).collect();


    let mut sensor_data: Vec<Vec<SensorValue>> = vec![Vec::new(); sensor_list.len()];

    // goes through the elements of divided vector

for part in data_split {
    let mut sbstr = divide_data(part);

    if sbstr.len() > 6 {
        for i in 0..sensor_list.len() {
            if sbstr.len() > 2 && sbstr[0] == id_to_hex(&sensor_list[i].id).unwrap() {
                sbstr.remove(0); // Remove the ID

                if sbstr.len() >= sensor_list[i].values.len() {
                    for (j, value_str) in sbstr.iter().enumerate().take(sensor_list[i].values.len()) {
                        match hex::decode(value_str) {
                            Ok(bytes) => {
                                let value = match sensor_list[i].values[j].type_ {
                                    DataType::F32 => {
                                        let mut padded = bytes.clone();
                                        // to don't resize
                                        padded.resize(4, 0); // pad to 4 bytes for f32
                                        SensorValue::F32(f32::from_le_bytes(padded.try_into().unwrap()))
                                    }
                                    DataType::U8 => {
                                        SensorValue::U8(bytes[0])
                                    }
                                    DataType::U16 => {
                                        let mut padded = bytes.clone();
                                        padded.resize(2, 0); // pad to 2 bytes for u16
                                        SensorValue::U16(u16::from_le_bytes(padded.try_into().unwrap()))
                                    }
                                    DataType::U32 => {
                                        let mut padded = bytes.clone();
                                        padded.resize(4, 0); // pad to 4 bytes for u32
                                        let cu = SensorValue::U32(u32::from_le_bytes(padded.try_into().unwrap()));

                                        println!("{:?} == {:02X?}", cu, bytes);
                                        cu
                                    }
                                };
                                sensor_data[i].push(value);
                            }
                            Err(e) => println!("Decode error: {}", e),
                        }
                    }
                }
            }
        }
    }
}



    for i in 0..sensor_list.len() {
    let path_name = format!("{directory}/{}.csv", sensor_list[i].name);
    let path = Path::new(&path_name);
    let mut sensor_file = csv_starter(&sensor_list[i], path).unwrap();
    let number_of_values = sensor_list[i].values.len();

    for (k, value) in sensor_data[i].iter().enumerate() {
        let csv = match value {
            SensorValue::F32(v) => format!("{:.4}", v),
            SensorValue::U8(v) => format!("{}", v),
            SensorValue::U16(v) => format!("{}", v),
            SensorValue::U32(v) => format!("{}", v),
        };

        if k % number_of_values == number_of_values - 1 {
            let csv = format!("{},\n", csv);
            let _ = sensor_file.write_all(csv.as_bytes());
        } else {
            let csv = format!("{},", csv);
            let _ = sensor_file.write_all(csv.as_bytes());
        }
    }
}
    Ok(())
}
