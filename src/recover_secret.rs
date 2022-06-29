use std::collections::HashMap;
use std::collections::HashSet;

fn generateAllSuccessorsForAnEntry(indexEntry: usize, entry: Vec<String>) -> HashSet<String> {
    let mut successors: HashSet<String> = HashSet::new();
    for i in 0..entry.len() {
        if i != indexEntry {
            successors.insert(entry[i].clone());
        }
    }
    return successors;
}


fn entriesToHashmap(entries: Vec<Vec<String>>) -> HashMap<String, HashSet<String>> {
    let mut map: HashMap<String, HashSet<String>> = HashMap::new();
    // iterate on entries with key and value
    for entry in entries {
        let mut set = HashSet::new();
        for (entriesOneEntry, oneEntry) in entry.iter().enumerate() {

            if map.contains_key(oneEntry) {

                let mut value_add;
                if let Ok(n) = map.get(oneEntry) {
                    value_add = n;
                }else {
                    value_add = &mut HashSet::new();
                }


                let mut value_present = map.get(oneEntry);
                if value_present != None {
                    value_present = HashSet::new();
                }
                let value_to_add = match entriesOneEntry >= entry.length - 1 {
                    true => {
                        []
                    }
                    false => {
                        generateAllSuccessorsForAnEntry(entriesOneEntry, entry);
                    }
                };
                //dict.set(entries[i][e], new Set([...value_to_add, ...value_present])); to rust
                set.insert(value_to_add);
                set.insert(value_present);
                map.insert(oneEntry, set);
            } else {
                let valueToAdd = match entriesOneEntry >= entry.length - 1 {
                    true => {
                        []
                    }
                    false => {
                        generateAllSuccessorsForAnEntry(entriesOneEntry, entry)
                    }
                };
                //dict.set(entries[i][e], new Set([...valueToAdd])); to rust
                set.insert(valueToAdd);
                map.insert(oneEntry, set);

            }
        }
    }

    return map;
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
    entriesToHashmap(vec![
        vec!["e".to_string(), "t".to_string(), "o".to_string().to_string()],
        vec!["c".to_string(), "o".to_string()],
        vec!["C".to_string(), "\'".to_string(), "t".to_string(), "o".to_string()],
        vec!["\'".to_string(), "h".to_string(), "u".to_string()],
        vec!["t".to_string(), "c".to_string(), "o".to_string()],
        vec!["o".to_string(), "u".to_string()],
    ]);
}