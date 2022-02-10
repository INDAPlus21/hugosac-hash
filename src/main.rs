use std::collections::LinkedList;
use std::error::Error;

use csv;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    pub key: String,
    pub value: String
}

struct HashTable {
    pub table: Vec<Option<LinkedList<Data>>>,
    pub num_of_lists: usize,
    pub num_of_elements: usize,
    pub capacity: usize
}

impl HashTable {
    pub fn new(capacity: usize) -> HashTable {
        let mut hash_table: Vec<Option<LinkedList<Data>>> = Vec::with_capacity(capacity);
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
                list.push_front(element);
                self.num_of_elements += 1;
            }
            None => { 
                self.table[index] = Some(LinkedList::from([element]));
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
                list.push_front(
                    Data { 
                        key: key.to_string(), 
                        value: value.to_string() 
                    }
                );
                self.num_of_elements += 1;
            }
            None => { 
                self.table[index] = Some(LinkedList::from(
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


fn main() -> Result<(), Box<dyn Error>> {
    let path = "data.csv";
    let _capacity = 10;
    let mut hash_table = HashTable::new(_capacity);

    read_csv(&mut hash_table, path).expect("Failed to read file");

    hash_table.insert(&String::from("Germany"), &String::from("Berlin"));

    
    write_csv(&mut hash_table, path).expect("Failed to write to file");

    let data = hash_table.get(&String::from("Sweden"));

    match data {
        Some(element) => { println!("{}", element.value) }
        None => {}
    }

    Ok(())
}
