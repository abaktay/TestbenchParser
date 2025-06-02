#![allow(dead_code)]
use parser::*;
use std::fs::File;
use std::io::{Read, Write};
use std::env;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let signal = "ff000000".repeat(4);

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
                //     );
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
    let mut adc1: Vec<f32> = Vec::new();
    let mut adc2: Vec<f32> = Vec::new();
    let mut adc3: Vec<f32> = Vec::new();


    // goes through the elements of divided vector
    println!("{:#?}", data_split[0]);
    
    for part in data_split {

       let mut sbstr = divide_data(part);

        if sbstr.len() > 2 && sbstr[0] == id_to_hex(&sensor_list[1].id).unwrap() {
            sbstr.remove(0); // Remove the ID after checking

            for i in 0..sbstr.len() {
                match hex::decode(&sbstr[i]) {
                    Ok(bytes) => {

                        let mut padded = bytes.clone();

                        padded.resize(4, 0);
                        let value = f32::from_le_bytes(padded.try_into().unwrap());
                        adc2.push(value.try_into().unwrap());

                        continue;
                    }
                    Err(e) => println!("Decode error: {}", e),
                }
            }
        } else if sbstr.len() > 2 && sbstr[0] == id_to_hex(&sensor_list[2].id).unwrap(){
            sbstr.remove(0); // Remove the ID after checking

            for i in 0..sbstr.len() {
                match hex::decode(&sbstr[i]) {
                    Ok(bytes) => {
                        let mut padded = bytes.clone();
                        padded.resize(4, 0);
                        let value = f32::from_le_bytes(padded.try_into().unwrap());
                        adc3.push(value);

                        continue;
                    }
                    Err(e) => println!("Decode error: {}", e),
                }
            }
        }

        if sbstr.len() > 2 && sbstr[0] == id_to_hex(&sensor_list[0].id).unwrap() {
            sbstr.remove(0); // Remove the ID after checking

            for i in 0..sbstr.len() {
                match hex::decode(&sbstr[i]) {
                    Ok(bytes) => {

                        let mut padded = bytes.clone();
                        padded.resize(4, 0);
                        let value: f32 = f32::from_le_bytes(padded.try_into().unwrap());
                        adc1.push(value);

                        continue;
                    }
                    Err(e) => println!("Decode error: {}", e),
                }
            }
        }

        // create new files using sensor vector
        let adc2_path_name = format!("{directory}/adc2.csv"); 
        let adc2_path = Path::new(&adc2_path_name);
        let mut test_adc2 = csv_starter(&sensor_list[1], adc2_path).unwrap();

        let adc2_number_of_sensors = sensor_list[1].values.len();
 
        for (i, v) in adc2.iter().enumerate() {
            if i % adc2_number_of_sensors == 0 {
                let csv = format!("{:.4},",v);
                let _ = test_adc2.write_all(csv.as_bytes());
            } else {
                let csv = format!("{v},\n");
                let _ = test_adc2.write_all(csv.as_bytes());
            }
        }

        let adc3_path_name = format!("{directory}/adc3.csv"); 
        let adc3_path = Path::new(&adc3_path_name);
 
        let mut test_adc3 = csv_starter(&sensor_list[1], adc3_path).unwrap();

        let adc3_number_of_sensors = sensor_list[1].values.len();

        for (i, v) in adc3.iter().enumerate() {
            if i % adc3_number_of_sensors == 0 {
                let csv = format!("{:.4},", v);
                let _ = test_adc3.write_all(csv.as_bytes());
            } else {
                let csv = format!("{v},\n");
                let _ = test_adc3.write_all(csv.as_bytes());
            }
        }
        let adc1_path_name = format!("{directory}/adc1.csv"); 
        let adc1_path = Path::new(&adc1_path_name);
 
        let mut test_adc1 = csv_starter(&sensor_list[0], adc1_path).unwrap();

        let adc1_number_of_sensors = sensor_list[0].values.len();

        for (i, v) in adc1.iter().enumerate() {
            if i % adc1_number_of_sensors == 3 {
                let csv = format!("{},\n", *v as u32);
                let _ = test_adc1.write_all(csv.as_bytes());
            } else {
                let csv = format!("{:.4},", v);
                let _ = test_adc1.write_all(csv.as_bytes());
            }
        }
    }
    Ok(())
}
