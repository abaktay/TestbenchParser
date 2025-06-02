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
    let mut sensor1: Vec<f32> = Vec::new();
    let mut sensor2: Vec<f32> = Vec::new();
    let mut sensor3: Vec<f32> = Vec::new();
    let mut sensor4: Vec<f32> = Vec::new();
    let mut sensor5: Vec<f32> = Vec::new();
    let mut sensor6: Vec<f32> = Vec::new();

    // goes through the elements of divided vector
    for part in data_split {
       let mut sbstr = divide_data(part);

       if sbstr.len() > 6 {
       
        for i in 0..sensor_list.len() {

            // test if ids check out
            //println!("{} -- {}", id_to_hex(&sensor_list[i].id).unwrap(), sbstr[0]) ;
            

            if sbstr.len() > 2 && sbstr[0] == id_to_hex(&sensor_list[i].id).unwrap() {
            sbstr.remove(0); // Remove the ID after checking

            for j in 0..sbstr.len() {
                match hex::decode(&sbstr[j]) {
                    Ok(bytes) => {
        
                        let mut padded = bytes.clone();

                        padded.resize(4, 0);
                        let value = f32::from_le_bytes(padded.try_into().unwrap());

                        match i {
                            0 => sensor1.push(value),
                            1 => sensor2.push(value),
                            2 => sensor3.push(value),
                            3 => sensor4.push(value),
                            4 => sensor5.push(value),
                            5 => sensor6.push(value),
                            _ => continue,
                        }
                        

                        continue;
                    }
                    Err(e) => println!("Decode error: {}", e),
                }
            }
        }

        }
        }
    
    }




    for i in 0..sensor_list.len(){

   
        let path_name = format!("{directory}/{}.csv", sensor_list[i].name); 
        let path = Path::new(&path_name);
        let mut sensor_file = csv_starter(&sensor_list[i], path).unwrap();
        let number_of_sensors = &sensor_list[i].values.len();

        if *number_of_sensors > 1 {

        match i {
            0 => {
                for (k, v) in sensor1.iter().enumerate() {
                    if k % number_of_sensors == number_of_sensors - 1 {
                        let csv = format!("{},\n", *v as u32);
                        let _ = sensor_file.write_all(csv.as_bytes());
                    } else {
                        let csv = format!("{},", *v as f32);
                        let _ = sensor_file.write_all(csv.as_bytes());
                    }
                }
            } 
            1 => {
                for (k, v) in sensor2.iter().enumerate() {
                    if k % number_of_sensors == number_of_sensors - 1 {
                        let csv = format!("{},\n", *v as u32);
                        let _ = sensor_file.write_all(csv.as_bytes());
                    } else {
                        let csv = format!("{},", *v as f32);
                        let _ = sensor_file.write_all(csv.as_bytes());
                    }
                }

            }
            2 => {
                for (k, v) in sensor3.iter().enumerate() {
                    if k % number_of_sensors == number_of_sensors - 1 {
                        let csv = format!("{},\n", *v as u32);
                        let _ = sensor_file.write_all(csv.as_bytes());
                    } else {
                        let csv = format!("{},", *v as f32);
                        let _ = sensor_file.write_all(csv.as_bytes());
                    }
                }
            }
            3 => {
                for (k, v) in sensor4.iter().enumerate() {
                    if k % number_of_sensors == number_of_sensors - 1 {
                        let csv = format!("{},\n", *v as u32);
                        let _ = sensor_file.write_all(csv.as_bytes());
                    } else {
                        let csv = format!("{},", *v as f32);
                        let _ = sensor_file.write_all(csv.as_bytes());
                    }
                }
            }
            4 => {
                for (k, v) in sensor5.iter().enumerate() {
                    if k % number_of_sensors == number_of_sensors - 1 {
                        let csv = format!("{},\n", *v as u32);
                        let _ = sensor_file.write_all(csv.as_bytes());
                    } else {
                        let csv = format!("{},", *v as f32);
                        let _ = sensor_file.write_all(csv.as_bytes());
                    }
                }
            }
            5 => {
                for (k, v) in sensor6.iter().enumerate() {
                    if k % number_of_sensors == number_of_sensors - 1 {
                        let csv = format!("{},\n", *v as u32);
                        let _ = sensor_file.write_all(csv.as_bytes());
                    } else {
                        let csv = format!("{},", *v as f32);
                        let _ = sensor_file.write_all(csv.as_bytes());
                    }
                }
            }
            _ => continue,
        }
       
        }

     
    }

    Ok(())
}
