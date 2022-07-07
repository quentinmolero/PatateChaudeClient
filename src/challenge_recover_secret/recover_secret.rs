use std::collections::HashMap;
use std::collections::HashSet;
#[allow(unused_imports)]
use crate::challenge::Challenge;
#[allow(unused_imports)]
use crate::challenge_recover_secret::recover_secret_challenge::Recover;

fn generate_all_successors_for_an_entry(index_entry: usize, entry: &Vec<String>) -> HashSet<String> {
    let mut successors: HashSet<String> = HashSet::new();
    for i in (index_entry + 1)..entry.len() {
        successors.insert(entry[i].clone());
    }
    return successors;
}


pub(crate) fn entries_to_hashmap(entries: Vec<Vec<String>>) -> HashMap<String, HashSet<String>> {
    let mut map: HashMap<String, HashSet<String>> = HashMap::new();
    for entry in entries {
        for (entries_one_entry, one_entry) in entry.iter().enumerate() {
            if map.contains_key(one_entry) {
                let value_present: HashSet<String>;
                if map.contains_key(one_entry) {
                    value_present = map.get(one_entry).unwrap().clone();
                }else {
                    value_present = HashSet::new();
                }

                let value_to_add: HashSet<String>;
                if entries_one_entry >= entry.len() - 1 {
                    value_to_add = HashSet::new();
                }else {
                    value_to_add = generate_all_successors_for_an_entry(entries_one_entry, &entry);
                }

                let mut set: HashSet<String> = HashSet::new();
                value_to_add.iter().for_each(|value| {
                    set.insert(value.clone());
                });
                value_present.iter().for_each(|value| {
                    set.insert(value.clone());
                });
                map.insert(one_entry.clone(), set);
            } else {
                let value_to_add: HashSet<String>;
                if entries_one_entry >= entry.len() - 1 {
                    value_to_add = HashSet::new();
                }else {
                    value_to_add = generate_all_successors_for_an_entry(entries_one_entry, &entry);
                }
                let mut set: HashSet<String> = HashSet::new();
                value_to_add.iter().for_each(|value| {
                    set.insert(value.clone());
                });
                map.insert(one_entry.clone(), set);

            }
        }
    }

    return map;
}

pub(crate) fn generate_string_from_hashmap(map: &HashMap<String, HashSet<String>>) -> String {
    let mut result: String = String::new();
    let mut map_copy: HashMap<String, HashSet<String>> = map.clone();
    while map_copy.len() > 0 {
        let mut elements_to_delete: Vec<String> = Vec::new();
        let mut map_elements_to_delete: Vec<String> = Vec::new();
        for (key, value) in map_copy.iter() {
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
            map_copy.iter_mut().for_each(|(_, v)| {
                if v.contains(key) {
                    v.remove(key);
                }
            });
        });
    }

    return result;
}


#[test]
fn should_return_map_with_only_one_entry() {
    let result = HashMap::from([
        ("e".to_string(), HashSet::from(["t".to_string()])),
        ("t".to_string(), HashSet::new()),
    ]);
    // create hashmmap with only one entry

    let entries = vec![
        vec!["e".to_string(), "t".to_string()],
        ];
    let hashmap_result = entries_to_hashmap(entries);

    assert_eq!(result, hashmap_result);
}

#[test]
fn should_return_map_with_two_entry_not_linked() {
    let result = HashMap::from([
        ("e".to_string(), HashSet::from(["t".to_string()])),
        ("t".to_string(), HashSet::new()),
        ("f".to_string(), HashSet::from(["g".to_string()])),
        ("g".to_string(), HashSet::new()),
    ]);
    // create hashmmap with only one entry

    let entries = vec![
        vec!["e".to_string(), "t".to_string()],
        vec!["f".to_string(), "g".to_string()],
    ];
    let hashmap_result = entries_to_hashmap(entries);

    assert_eq!(result, hashmap_result);
}

#[test]
fn should_return_map_with_two_entry_linked() {
    let result = HashMap::from([
        ("e".to_string(), HashSet::from(["t".to_string()])),
        ("t".to_string(), HashSet::from(["o".to_string()])),
        ("o".to_string(), HashSet::new()),
    ]);
    // create hashmmap with only one entry

    let entries = vec![
        vec!["e".to_string(), "t".to_string()],
        vec!["t".to_string(), "o".to_string()],
    ];
    let hashmap_result = entries_to_hashmap(entries);

    assert_eq!(result, hashmap_result);
}

#[test]
fn should_generate_string_with_hashmap_with_no_entry() {
    let result = "";
    let map = HashMap::from([]);

    let string_generate = generate_string_from_hashmap(&map);

    assert_eq!(result, string_generate);
}

#[test]
fn should_generate_string_with_hashmap_with_one_entry() {
    let result = "t";
    let map = HashMap::from([
        ("t".to_string(), HashSet::new()),
    ]);

    let string_generate = generate_string_from_hashmap(&map);

    assert_eq!(result, string_generate);
}

#[test]
fn should_generate_string_with_hashmap_with_two_entry_linked_by_order() {
    let result = "to";
    let map = HashMap::from([
        ("t".to_string(), HashSet::from(['o'.to_string()])),
        ("o".to_string(), HashSet::new()),
    ]);

    let string_generate = generate_string_from_hashmap(&map);

    assert_eq!(result, string_generate);
}

#[test]
fn should_generate_string_with_hashmap_with_three_entry_linked_by_order() {
    let result = "efgt";
    let map = HashMap::from([
        ("e".to_string(), HashSet::from(["t".to_string(), "f".to_string()])),
        ("t".to_string(), HashSet::new()),
        ("f".to_string(), HashSet::from(["g".to_string()])),
        ("g".to_string(), HashSet::from(["t".to_string()])),
    ]);

    let string_generate = generate_string_from_hashmap(&map);

    assert_eq!(result, string_generate);
}

#[test]
fn should_generate_string_with_hashmap_with_an_acceptance_test() {
    let result = "c'estChou";
    let map = HashMap::from([
        ("c".to_string(), HashSet::from(["\'".to_string()])),
        ("\'".to_string(), HashSet::from(["u".to_string(), "e".to_string()])),
        ("C".to_string(), HashSet::from(["h".to_string()])),
        ("h".to_string(), HashSet::from(["o".to_string(), "u".to_string()])),
        ("s".to_string(), HashSet::from(["t".to_string(), "u".to_string()])),
        ("t".to_string(), HashSet::from(["C".to_string(), "o".to_string()])),
        ("o".to_string(), HashSet::from(["u".to_string()])),
        ("u".to_string(), HashSet::new()),
        ("e".to_string(), HashSet::from(["s".to_string()])),
    ]);

    let string_generate = generate_string_from_hashmap(&map);

    assert_eq!(result, string_generate);
}


#[test]
fn should_return_right_response_for_complexity_0() {
    let recover_secret_input = crate::challenge_message::RecoverSecretInput {
        word_count: 2,
        letters: "'e otcouCesc's ost cuC'eu'etch".to_string(),
        tuple_sizes: [4, 4, 4, 4, 5, 4, 5].to_vec(),
    };

    let recover_secret = Recover::new(recover_secret_input);
    let recover_secret_result = &Recover::solve(&recover_secret);

    let array_result = [
        "C'est cohu".to_string(),
        "C'est couh".to_string(),
    ];

    assert_eq!(true, array_result.contains(&recover_secret_result.secret_sentence));
}

