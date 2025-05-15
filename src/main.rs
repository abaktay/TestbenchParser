#![allow(dead_code)]
use parser::*;
use std::fs::File;
use std::io::{Read, Write};
use std::env;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let signal = "ff000000ff000000ff000000ff000000";

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

    //file.read_to_end(&mut data)?;

    // println!("{:02x?}", data);
    // turns binary to string in hexadecimal format
    let hex_string: String = data.iter().map(|byte| format!("{:02x}", byte)).collect();



//     println!("{}", hex_string);
    
    // splits data with the end-of-data signal as a vector
    let data_split: Vec<&str> = hex_string.split(&signal).collect();
//    println!("{:#?}", data_split);
    let mut adc1: Vec<f32> = Vec::new();
    let mut adc2: Vec<f32> = Vec::new();
    let mut adc3: Vec<f32> = Vec::new();

    assert_eq!("b0000000", id_to_hex(11));
    // goes through the elements of divided vector
    //data_split.remove(1);
    // println!("{:#?}", data_split);
    for part in data_split {

       let mut sbstr = divide_data(part);

//       println!("{:#?}", part.len());
        // println!("{:#?}", sbstr);
        // println!("{}", sbstr[4]);
        if sbstr.len() > 2 && sbstr[0] == id_to_hex(22) {
            for _ in 0..1 {
                sbstr.remove(0); // Remove the ID after checking
            }
            // println!("{:#?}", sbstr);

            // println!("Hello from ADC2");
            for i in 0..sbstr.len() {
                match hex::decode(&sbstr[i]) {
                    Ok(bytes) => {
                        // println!("{}", sbstr[1]);
                        // println!("{:#?}", sbstr[i]);

                        let mut padded = bytes.clone();
                        // println!("{:X?}", padded);

                        padded.resize(4, 0);
                        let value = f32::from_le_bytes(padded.try_into().unwrap());
                        // let value = hex_to_dec(padded);
                        // println!("{:#?}", value);
                        adc2.push(value.try_into().unwrap());

                        continue;
                    }
                    Err(e) => println!("Decode error: {}", e),
                }
            }
        } else if sbstr.len() > 2 && sbstr[0] == "21000000" {
            for _ in 0..1 {
                sbstr.remove(0); // Remove the ID after checking
            }
            // println!("{:#?}", sbstr);

            // println!("Hello from ADC3");

            for i in 0..sbstr.len() {
                match hex::decode(&sbstr[i]) {
                    Ok(bytes) => {
                        // println!("{}", sbstr[1]);
                        // println!("{:#?}", sbstr[i]);

                        let mut padded = bytes.clone();
                        // println!("{:#?}", bytes);
                        padded.resize(4, 0);
                        let value = f32::from_le_bytes(padded.try_into().unwrap());
                        adc3.push(value);

                        continue;
                    }
                    Err(e) => println!("Decode error: {}", e),
                }
            }
        }

        if sbstr.len() > 2 && sbstr[0] == "0b000000" {
            for _ in 0..1 {
                sbstr.remove(0); // Remove the ID after checking
            }
            // println!("{:#?}", sbstr);

            // println!("Hello from ADC1");

            for i in 0..sbstr.len() {
                match hex::decode(&sbstr[i]) {
                    Ok(bytes) => {
                        // println!("{}", sbstr[1]);
                        // println!("{:#?}", sbstr[i]);

                        let mut padded = bytes.clone();
                        // println!("{:#?}", bytes);
                        padded.resize(4, 0);
                        let value: f32 = f32::from_le_bytes(padded.try_into().unwrap());
                        // let value = hex_to_dec(padded);
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
        // println!("{adc2_number_of_sensors}");
        //adc2.remove(0);
        for (i, v) in adc2.iter().enumerate() {
            // println!("{i}: {v}");
            if i % adc2_number_of_sensors == 0 {
                let csv = format!("{:.4},",v);
                let _ = test_adc2.write_all(csv.as_bytes());
            } else {
                let csv = format!("{v},\n");
                let _ = test_adc2.write_all(csv.as_bytes());
            }
        }

        // Test data doesn't include ADC3, but it should work once it's added
        let adc3_path_name = format!("{directory}/adc3.csv"); 
        let adc3_path = Path::new(&adc3_path_name);
 
        let mut test_adc3 = csv_starter(&sensor_list[1], adc3_path).unwrap();

        let adc3_number_of_sensors = sensor_list[1].values.len();

        for (i, v) in adc3.iter().enumerate() {
            // println!("{i}: {v}");
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

        // println!("ADC1 SENSOR{adc1_number_of_sensors}");
        for (i, v) in adc1.iter().enumerate() {
            // println!("{i}: {v}");
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
