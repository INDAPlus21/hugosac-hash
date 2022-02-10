use std::collections::LinkedList;

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

    pub fn get(&mut self, key: &String) -> String {
        let index = hashcode(key) % self.capacity;
        match &mut self.table[index] {
            Some(list) => {
                for element in list.iter() {
                    if &element.key == key {
                        return element.value.to_string();
                    }
                }
                return String::from("Key is not present in the database");
            }
            None => {
                return String::from("Key is not present in the database");
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

            _ => {
                return -3;
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


fn main() {

    let capacity = 3;
    let mut hash_table = HashTable::new(capacity);
    hash_table.insert(&String::from("Hugo"), &String::from("Sacilotto"));
    hash_table.insert(&String::from("Hugo"), &String::from("Sacilotto"));
    println!("{}", hash_table.get_index(&String::from("Hugo")));
    hash_table.insert(&String::from("Hug"), &String::from("Saci"));
    hash_table.insert(&String::from("go"), &String::from("Saci"));
    hash_table.insert(&String::from("ugo"), &String::from("Saci"));
    hash_table.insert(&String::from("Hu"), &String::from("Saci"));
    hash_table.insert(&String::from("Huuugo"), &String::from("Saci"));
    hash_table.insert(&String::from("Huggo"), &String::from("Saci"));
    hash_table.insert(&String::from("Huggggo"), &String::from("Saci"));
    println!("{}", hash_table.get_index(&String::from("Hugo")));
    
    
    // let value = hash_table.get(&String::from("Hugo"));
    // println!("{}", value);
    // println!("{}", hash_table.num_of_elements);
    // println!("{}", hash_table.num_of_lists);
}