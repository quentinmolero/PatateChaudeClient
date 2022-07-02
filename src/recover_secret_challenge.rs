use crate::challenge::Challenge;
use crate::challenge_message::{RecoverSecretInput, RecoverSecretOutput};
use crate::recover_secret::{add_space, entries_to_hashmap, generate_string_from_hashmap};

pub(crate) struct Recover {
    input: RecoverSecretInput,
}

impl Challenge for Recover {
    type Input = RecoverSecretInput;
    type Output = RecoverSecretOutput;

    fn name() -> String {
        String::from("Recover")
    }

    fn new(input: Self::Input) -> Recover {
        Recover {
            input,
        }
    }

    fn solve(&self) -> RecoverSecretOutput {
        let entries = recover_secret_challenge_to_entries(RecoverSecretInput { word_count: self.input.word_count.clone(), letters: self.input.letters.clone(), tuple_sizes: self.input.tuple_sizes.clone() });
        let map = entries_to_hashmap(entries);
        let result = generate_string_from_hashmap(&map);
        //let result_space = addSpace(&result, self.input.word_count);
        return RecoverSecretOutput {
            secret_sentence: result.to_string(),
        };
    }

    fn verify(&self, _: Self::Output) -> bool {
        todo!()
    }
}

fn recover_secret_challenge_to_entries(recover_secret_challenge: RecoverSecretInput) -> Vec<Vec<String>> {
    let mut entries: Vec<Vec<String>> = Vec::new();
    let letters_split = recover_secret_challenge.letters.split("");
    let letters_vec = letters_split.collect::<Vec<&str>>();
    let tuple_sizes = recover_secret_challenge.tuple_sizes.clone();
    let mut index: usize = 1;
    tuple_sizes.iter().for_each(|tuple_size| {
        let mut entry: Vec<String> = Vec::new();
        for _ in 0..*tuple_size {
            entry.push(letters_vec[index].to_string());
            index+=1;
        }
        entries.push(entry);
    });
    return entries;
}

#[test]
fn should_return_empty_entries_because_empty_tuple() {
    let recover_secret = RecoverSecretInput { word_count: 1, letters: "zFSZPdsYveFSIELYQ9FwIs6NqB3wnkjoect1z".to_string(), tuple_sizes: Vec::from([])};
    let result_expected: Vec<Vec<String>> = Vec::from([]);

    let result = recover_secret_challenge_to_entries(recover_secret);


    assert_eq!(result_expected, result);
}

#[test]
fn should_map_complexe_recover_secret_entry_to_entries() {
    let recover_secret = RecoverSecretInput {word_count: 1, letters: "zFSZPdsYveFSIELYQ9FwIs6NqB3wnkjoect1z".to_string(), tuple_sizes: Vec::from([9, 8, 8, 7, 5])};
    let result_expected: Vec<Vec<String>> = Vec::from([
        Vec::from(["z".to_string(),"F".to_string(),"S".to_string(),'Z'.to_string(),'P'.to_string(),'d'.to_string(), 's'.to_string(), 'Y'.to_string(),'v'.to_string()]),
        Vec::from(['e'.to_string(),'F'.to_string(),'S'.to_string(),'I'.to_string(),'E'.to_string(),'L'.to_string(),'Y'.to_string(),'Q'.to_string()]),
        Vec::from(['9'.to_string(),'F'.to_string(),'w'.to_string(),'I'.to_string(),'s'.to_string(),'6'.to_string(),'N'.to_string(),'q'.to_string()]),
        Vec::from(['B'.to_string(),'3'.to_string(),'w'.to_string(),'n'.to_string(),'k'.to_string(),'j'.to_string(),'o'.to_string()]),
        Vec::from(['e'.to_string(),'c'.to_string(),'t'.to_string(),'1'.to_string(),'z'.to_string()])
    ]);

    let result = recover_secret_challenge_to_entries(recover_secret);


    assert_eq!(result_expected, result);
}
