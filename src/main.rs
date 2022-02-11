use std::error::Error;
use std::io;

use csv;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    pub country: String,
    pub capital: String
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
        let index: usize = hashcode(&element.country) % self.capacity;

        if self.num_of_lists == self.capacity - 1 {
            println!("Expansion is needed");
            // Expand table
            self.capacity *= 2;
            let mut new_table = HashTable::new(self.capacity);

            // Copy all elements 
            for _list in &self.table {
                match _list {
                    Some(list) => {
                        for entry in list.iter() {
                            new_table.insert(&entry.country, &entry.capital);
                        }
                    }
                    None => {}
                }
            }

            *self = new_table;
            println!("Table has been rebuilt!");
        }

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

    pub fn insert(&mut self, country: &String, capital: &String) {
        // Check if country is already used
        if self.contains(country) {
            println!("{} is already in the data base\n", country);
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
                            new_table.insert(&element.country, &element.capital);
                        }
                    }
                    None => {}
                }
            }
            *self = new_table;
            println!("Table has been rebuilt!");
        }

        let index: usize = hashcode(country) % self.capacity; 

        match &mut self.table[index] {
            Some(list) => {
                list.push(
                    Data { 
                        country: country.to_string(), 
                        capital: capital.to_string() 
                    }
                );
                self.num_of_elements += 1;
            }
            None => { 
                self.table[index] = Some(Vec::from(
                    [
                        Data {
                            country: country.to_string(),
                            capital: capital.to_string()
                        }
                    ]
                ));
                self.num_of_lists += 1;
                self.num_of_elements += 1;
            }
        }
        println!("Successfully inserted {}\n", country);
    }

    pub fn get(&mut self, country: &String) -> Option<Data> {
        let index = hashcode(country) % self.capacity;
        match &self.table[index] {
            Some(list) => {
                for element in list.iter() {
                    if &element.country == country {
                        return Some(
                            Data {
                                country: element.country.to_string(),
                                capital: element.capital.to_string()
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

    pub fn delete(&mut self, country: &String) {
        for _i in 0..self.capacity {
            match &mut self.table[_i] {
                Some(list) => {
                    for _j in 0..list.len() {
                        if &list[_j].country == country {
                            let _ = list.remove(_j);
                            println!("Successfully deleted {}\n", country);
                            return;
                        } 
                    }
                }
                None => { }
            }
        }
        println!("Element not found\n");
    }

    pub fn contains(&mut self, country: &String) -> bool {
        let index = hashcode(country) % self.capacity;
        match &mut self.table[index] {
            Some(list) => {
                for element in list.iter() {
                    if &element.country == country {
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

    pub fn print_all(&mut self) {
        if self.capacity == 0 {
            println!("Table is empty\n");
            return;
        }
        for _i in 0..self.capacity {
            match &mut self.table[_i] {
                Some(list) => {
                    for element in list.iter() {
                        println!("{:?}", element);
                    }
                }
                None => {}
            }
        }
        println!("");
    }
}


fn hashcode(country: &String) -> usize {
    let mut constant: u32 = 1;

    let mut code: u32 = 0;

    for c in country.chars() {
        code += c as u32 * constant;
        constant *= 10;
    }

    return code as usize;
}

fn read_csv(hash_table: &mut HashTable, path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?; 

    for result in reader.deserialize() {
        let record: Data = result?;
        hash_table.add(record);
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
    let mut hash_table = HashTable::new(10);

    let input = io::stdin();

    read_csv(&mut hash_table, path).expect("Failed to read file");

    loop {
        let mut buffer = String::new();
        input.read_line(&mut buffer).expect("Couldn't read line");

        let args: Vec<&str> = buffer
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .collect();

        
        if args.len() < 1 { break; }

        match args[0] {
            "INSERT" => { 
                if args.len() < 3 {
                    println!("You need to specify country and capital\n");
                    continue;
                }
                hash_table.insert(&args[1].to_string(), &args[2].to_string()); 
            }
            "DELETE" => { 
                if args.len() < 2 {
                    println!("You need to specify a country to delete\n");
                    continue;
                }
                hash_table.delete(&args[1].to_string()); 
            }
            "ALL"    => { hash_table.print_all(); }
            "GET"    => {
                if args.len() < 2 {
                    println!("You need to specify a country to delete\n");
                    continue;
                }
                let data = hash_table.get(&args[1].to_string());
                match data {
                    Some(element) => { println!("{}\n", element.capital); }
                    None => { println!("Element not found\n"); }
                }
                continue;   // No need to rewrite to the csv file
            }
            "QUIT"   => { break; }
            _        => { println!("Invalid command\n"); }
        }

        write_csv(&mut hash_table, path).expect("Failed to write to file");
    }
}
