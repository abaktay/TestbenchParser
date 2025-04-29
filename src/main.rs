#![allow(dead_code)]
use parser::*;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    let signal = "1f".repeat(4);

    let mut sensor_list: Vec<Sensor> = Vec::new();
    let mut file = File::open("log.bin")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    match get_sensor(buffer) {
        Ok(sensors) => {
            for sensor in sensors {
                sensor_list.push(sensor.clone());
                println!("Sensor: {} (ID: {})", sensor.name, sensor.id);
                // csv_starter(&sensor, file_path);
                for value in sensor.values {
                    println!(
                        "  Value: {}, Type: {}, Unit: {}",
                        value.var_name, value.type_, value.unit
                    );
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    let mut file = File::open("zrt.bin")?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    // turns binary to string in hexadecimal format
    let hex_string: String = data.iter().map(|byte| format!("{:02x}", byte)).collect();

    // splits data with the end-of-data signal as a vector
    let data_split: Vec<&str> = hex_string.split(&signal).collect();

    let mut adc2: Vec<u32> = Vec::new();
    let mut adc3: Vec<u32> = Vec::new();

    // assert_eq!("160000", id_to_hex(22));
    // goes through the elements of divided vector
    for part in data_split {
        // println!("{:?}", part);
        // this divides the string in to hexadecimals of length 6
        // TODO this has to be in 8, fix the firmware code
        let sbstr = divide_data(part);
        // println!("{:?}", sbstr);

        for num in &sbstr {
            // println!("Hex string: {}", num);

            // checks for the id
            // BURADA BİŞİ VAR HA
            // TODO there's a function to do that
            if sbstr[0] == id_to_hex(22) {
                match hex::decode(&num) {
                    Ok(bytes) => {
                        // Padding with zeros if it's less than 4 bytes
                        let mut padded = bytes.clone();
                        padded.resize(4, 0); // Ensure we have exactly 4 bytes

                        let value = u32::from_le_bytes(padded.try_into().unwrap());
                        // println!("u32 value: {}", value);
                        // println!("Hex representation: {:06X}", value);
                        // println!("Bytes: {:02X?}", bytes);
                        adc2.push(value);
                    }
                    Err(e) => println!("Decode error: {}", e),
                }
            }

            if sbstr[0] == id_to_hex(33) {
                match hex::decode(&num) {
                    Ok(bytes) => {
                        // Padding with zeros if it's less than 4 bytes
                        let mut padded = bytes.clone();
                        padded.resize(4, 0);

                        let value = u32::from_le_bytes(padded.try_into().unwrap());
                        // println!("u32 value: {}", value);
                        // println!("Hex representation: {:06X}", value);
                        // println!("Bytes: {:02X?}", bytes);
                        adc3.push(value);
                    }
                    Err(e) => println!("Decode error: {}", e),
                }
            }
        }
    }

    let mut file_adc2 = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("../adc2.txt")?;

    // TODO implement an actual writing mechanism LOL
    for (i, v) in adc2.iter().enumerate() {
        // println!("{i}: {v}");
        let csv = format!("{i},{v}\n");
        file_adc2.write_all(csv.as_bytes())?;
    }

    let mut file_adc3 = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("../adc3.txt")?;

    for (i, v) in adc3.iter().enumerate() {
        // println!("{i}: {v}");
        let csv = format!("{i},{v}\n");
        file_adc3.write_all(csv.as_bytes())?;
    }
    Ok(())
}
