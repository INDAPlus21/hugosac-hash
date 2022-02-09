use std::collections::LinkedList;


struct Data {
    pub key: String,
    pub value: String
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




fn insert(hash_table: &mut Vec<Option<LinkedList<Data>>>, key: &String, value: &String) {
    // if hash_table.len() == hash_table.capacity() - 1 {
    //     // Expand table
    //     // Move all entries to new hash table

    //     // Copy table
    //     let mut hash_table_copy: Vec<LinkedList<Data>> = Vec::with_capacity(hash_table.capacity() * 2);

    //     // Add all data to the copy with new hashing indices
    //     for _i in 0..hash_table.len() {
    //         if hash_table[_i].len() > 0 {
    //             for element in hash_table[_i].iter() {
    //                 let _index = hashcode(&element.key) % hash_table_copy.capacity();
    //                 let data = element;
    //                 hash_table_copy[_index].push_front(*data);
    //             }
    //         }
    //     }

    //     hash_table.clear();
    //     let new_table = hash_table_copy;
    //     // hash_table = hash_table_copy;
    // }

    let index: usize = hashcode(key) % hash_table.capacity();

    match &mut hash_table[index] {
        Some(inner) => {
            inner.push_front(
                Data { key: key.to_string(), value: value.to_string() }
            );           
        },
        None => {
            hash_table[index] = Some(LinkedList::from(
                [
                    Data {
                        key: key.to_string(),
                        value: value.to_string()
                    }
                ]
            ));
        }
    }

}

fn main() {

    // let mut hash_table: Vec<LinkedList<Data>> = Vec::with_capacity(10);
    let mut hash_table: Vec<Option<LinkedList<Data>>> = Vec::with_capacity(20);
    for _ in 0..hash_table.capacity() {
        hash_table.push(None);
    }
    // println!("{}", hash_table.capacity());
    insert(&mut hash_table, &String::from("Hugo"), &String::from("Sacilotto"));
    insert(&mut hash_table, &String::from("Axel"), &String::from("Andersson"));
    // println!("{}", hash_table.len());

    println!("{}", hash_table[hashcode(&String::from("Hugo")) % hash_table.capacity()].as_ref().unwrap().iter().next().unwrap().value);
    println!("{}", hash_table[hashcode(&String::from("Axel")) % hash_table.capacity()].as_ref().unwrap().iter().next().unwrap().value);

}