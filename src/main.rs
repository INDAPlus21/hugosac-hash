use std::collections::LinkedList;
use std::error::Error;
use std::io;

use csv;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    pub key: String,
    pub value: String
}

struct HashTable {
    pub table: Vec<Option<Vec<Data>>>,
    pub num_of_lists: usize,
    pub num_of_elements: usize,
    pub capacity: usize
}

impl HashTable {
    pub fn new(capacity: usize) -> HashTable {
        let mut hash_table: Vec<Option<Vec<Data>>> = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            hash_table.push(None);
        }
        
        HashTable {
            table: hash_table,
            num_of_lists: 0,
            num_of_elements: 0,
            capacity: capacity
        }
    }

    pub fn add(&mut self, element: Data) {
        let index: usize = hashcode(&element.key) % self.capacity;

        match &mut self.table[index] {
            Some(list) => {
                list.push(element);
                self.num_of_elements += 1;
            }
            None => { 
                self.table[index] = Some(Vec::from([element]));
                self.num_of_lists += 1;
                self.num_of_elements += 1;
            }
        }
    }

    pub fn insert(&mut self, key: &String, value: &String) {
        // Check if key is already used
        if self.contains(key) {
            println!("{} is already in the data base", key);
            return;
        }
        
        if self.num_of_lists == self.capacity - 1 {
            println!("Expansion is needed");
            // Expand table
            self.capacity *= 2;
            let mut new_table = HashTable::new(self.capacity);

            // Copy all elements 
            for _list in &self.table {
                match _list {
                    Some(list) => {
                        for element in list.iter() {
                            new_table.insert(&element.key, &element.value);
                        }
                    }
                    None => {}
                }
            }

            *self = new_table;
            println!("Table has been rebuilt!");
        }

        let index: usize = hashcode(key) % self.capacity; 

        match &mut self.table[index] {
            Some(list) => {
                list.push(
                    Data { 
                        key: key.to_string(), 
                        value: value.to_string() 
                    }
                );
                self.num_of_elements += 1;
            }
            None => { 
                self.table[index] = Some(Vec::from(
                    [
                        Data {
                            key: key.to_string(),
                            value: value.to_string()
                        }
                    ]
                ));
                self.num_of_lists += 1;
                self.num_of_elements += 1;
            }
        }
    }

    pub fn get(&mut self, key: &String) -> Option<Data> {
        let index = hashcode(key) % self.capacity;
        match &self.table[index] {
            Some(list) => {
                for element in list.iter() {
                    if &element.key == key {
                        return Some(
                            Data {
                                key: element.key.to_string(),
                                value: element.value.to_string()
                            }
                        );
                    }
                }
                return None;
            }
            None => {
                return None;
            }
        }
    }

    pub fn delete(&mut self, key: &String) {
        for _i in 0..self.capacity {
            match &mut self.table[_i] {
                Some(list) => {
                    let mut _j = 0;
                    for element in list.iter() {
                        if &element.key == key {
                            list.remove(_j);
                            println!("Successfully deleted {}", key);
                            return;
                        } 
                        _j += 1;
                    }
                }
                None => { println!("Element not found"); }
            }
        }
    }

    pub fn contains(&mut self, key: &String) -> bool {
        let index = hashcode(key) % self.capacity;
        match &mut self.table[index] {
            Some(list) => {
                for element in list.iter() {
                    if &element.key == key {
                        return true;
                    }
                }
                return false;
            }
            None => {
                return false;
            }
        }
    }

    pub fn get_index(&mut self, key: &String) -> i32 {
        let index = hashcode(key) % self.capacity;
        match &mut self.table[index] {
            Some(list) => {
                for element in list.iter() {
                    if &element.key == key {
                        return index as i32;
                    }
                }
                return -1;
            }
            None => {
                return -2;
            }
        }
    }

    pub fn print_all(&mut self) {
        for _i in 0..self.capacity {
            match &mut self.table[_i] {
                Some(list) => {
                    for element in list.iter() {
                        println!("{:?}", element)
                    }
                }
                None => {}
            }
        }
    }
}


fn hashcode(key: &String) -> usize {
    let mut constant: u32 = 1;

    let mut code: u32 = 0;

    for c in key.chars() {
        code += c as u32 * constant;
        constant *= 10;
    }

    return code as usize;

}

fn read_csv(hash_table: &mut HashTable, path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?; 

    for result in reader.deserialize() {
        let record: Data = result?;
        hash_table.insert(&record.key, &record.value);
    }

    Ok(())
}

fn write_csv(hash_table: &mut HashTable, path: &str) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_path(path)?;

    for _i in 0..hash_table.capacity {
        match &mut hash_table.table[_i] {
            Some(list) => {
                for element in list.iter() {
                    writer.serialize(element)?;
                }
            }
            None => {}
        }
    }

    writer.flush()?;

    Ok(())
}


fn main() {
    let path = "data.csv";
    let _capacity = 10;
    let mut hash_table = HashTable::new(_capacity);

    let input = io::stdin();

    read_csv(&mut hash_table, path).expect("Failed to read file");

    loop {
        let mut buffer = String::new();
        input.read_line(&mut buffer).expect("Couldn't read line");

        let args: Vec<&str> = buffer
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .collect();

        // if args.len() < 3 { break; }

        match args[0] {
            "INSERT" => { hash_table.insert(&args[1].to_string(), &args[2].to_string()) }
            "DELETE" => { hash_table.delete(&args[1].to_string()) }
            "ALL"    => { hash_table.print_all() }
            "GET"    => {
                let data = hash_table.get(&args[1].to_string());
                match data {
                    Some(element) => { println!("{}", element.value); }
                    None => { println!("Element not found"); }
                }
            }
            "QUIT"   => { break; }
            _        => {}
        }

        write_csv(&mut hash_table, path).expect("Failed to write to file");
    }

}
