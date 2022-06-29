use std::collections::HashMap;
use std::collections::HashSet;

fn generateAllSuccessorsForAnEntry(indexEntry: usize, entry: &Vec<String>) -> HashSet<String> {
    let mut successors: HashSet<String> = HashSet::new();
    for i in (indexEntry + 1)..entry.len() {
        successors.insert(entry[i].clone());
    }
    return successors;
}


fn entriesToHashmap(entries: Vec<Vec<String>>) -> HashMap<String, HashSet<String>> {
    let mut map: HashMap<String, HashSet<String>> = HashMap::new();
    // iterate on entries with key and value
    for entry in entries {
        for (entriesOneEntry, oneEntry) in entry.iter().enumerate() {
            if map.contains_key(oneEntry) {
                let value_present: HashSet<String>;
                if map.contains_key(oneEntry) {
                    value_present = map.get(oneEntry).unwrap().clone();
                }else {
                    value_present = HashSet::new();
                }

                let value_to_add: HashSet<String>;
                if entriesOneEntry >= entry.iter().clone().len() - 1 {
                    value_to_add = HashSet::new();
                }else {
                    value_to_add = generateAllSuccessorsForAnEntry(entriesOneEntry, &entry);
                }

                let mut set: HashSet<String> = HashSet::new();
                value_to_add.iter().for_each(|value| {
                    set.insert(value.clone());
                });
                value_present.iter().for_each(|value| {
                    set.insert(value.clone());
                });
                map.insert(oneEntry.clone(), set);
            } else {
                let value_to_add: HashSet<String>;
                if entriesOneEntry >= entry.iter().len() - 1 {
                    value_to_add = HashSet::new();
                }else {
                    value_to_add = generateAllSuccessorsForAnEntry(entriesOneEntry, &entry);
                }
                let mut set: HashSet<String> = HashSet::new();
                value_to_add.iter().for_each(|value| {
                    set.insert(value.clone());
                });
                map.insert(oneEntry.clone(), set);

            }
        }
    }

    return map;
}

fn generateStringFromHashMap(map: &HashMap<String, HashSet<String>>) -> String {
    let mut result: String = String::new();
    // copy map
    let mut map_copy: HashMap<String, HashSet<String>> = map.clone();
    while map_copy.len() > 0 {
        let mut elements_to_delete: Vec<String> = Vec::new();
        let mut map_elements_to_delete: Vec<String> = Vec::new();
        for (key, value) in map_copy.iter_mut() {
            if value.len() == 0 {
                result = key.clone() + &result;
                map_elements_to_delete.push(key.clone());
                elements_to_delete.push(key.clone());
            }
        }
        map_elements_to_delete.iter().for_each(|key| {
            map_copy.remove(key);
        });
        elements_to_delete.iter().for_each(|key| {
            map_copy.iter_mut().for_each(|(k, v)| {
                if v.contains(key) {
                    v.remove(key);
                }
            });
        });
    }

    return result;
}

// create Vectors of Vectors of Strings
/*let entries = vec![
    vec!['e', 't', 'o'],
    vec!['c', 'o'],
    vec!['C', '\'', 't', 'o'],
    vec!['\'', 'h', 'u'],
    vec!['t', 'c', 'o'],
    vec!['o', 'u'],
];*/

fn main() {
    let result = entriesToHashmap(vec![
        vec!["e".to_string(), "t".to_string(), "o".to_string().to_string()],
        vec!["c".to_string(), "o".to_string()],
        vec!["C".to_string(), "\'".to_string(), "t".to_string(), "o".to_string()],
        vec!["\'".to_string(), "h".to_string(), "u".to_string()],
        vec!["t".to_string(), "c".to_string(), "o".to_string()],
        vec!["o".to_string(), "u".to_string()],
        vec!["s".to_string(), "c".to_string()],
    ]);

    println!("{:?}", result);

    let result_generate = generateStringFromHashMap(&result);
    println!("{:?}", result_generate);
}