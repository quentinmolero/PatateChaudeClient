use std::ptr::{eq, hash};
use std::process::{Command, Stdio};
use std::str::from_utf8;
use std::sync::{Arc, LockResult, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use crate::challenge::Challenge;
use crate::challenge_message::{MD5HashCashInput, MD5HashCashOutput};

extern crate num_cpus;

pub(crate) struct HashCash {
    input: MD5HashCashInput,
}

impl Challenge for HashCash {
    type Input = MD5HashCashInput;
    type Output = MD5HashCashOutput;

    fn name() -> String {
        String::from("HashCash")
    }

    fn new(input: Self::Input) -> HashCash {
        HashCash {
            input
        }
    }

    fn solve(&self) -> MD5HashCashOutput {
        let threads: usize = num_cpus::get();

        println!("{:}", num_cpus::get());
        let (tx, rx): (Sender<MD5HashCashOutput>, Receiver<MD5HashCashOutput>) = mpsc::channel();
        let mut i : u64 = 0;
        loop {
            if start_challenge(i, self.input.complexity.clone(), self.input.message.clone(), tx.clone()) {
                break;
            }
            i += 1;
        }

        for message in rx.iter().take(threads) {
            if message.hashcode != "ERR" {
                return message;
            }
        }

        return MD5HashCashOutput {
            seed : 0,
            hashcode: "ERR".parse().unwrap()
        }
    }

    fn verify(&self, answer: Self::Output) -> bool {
        todo!()
    }
}

trait CommandOutput {
    fn get_command_output() -> Result<(), ()>;
}

fn start_challenge(i : u64, complexity: u32, message : String, tx : Sender<MD5HashCashOutput>) -> bool {
    println!("Testing with {:}", i);

    let hash = format!("{:016X}", i) + &message;
    print!("{:?} = ", hash);

    let result = format!("{:x}", md5::compute(hash));
    print!("{:?} ", result);

    let binary   = u128::from_str_radix(&*result, 16).unwrap();
    let mut check = format!("{:0128b}", binary);
    let check2 : Vec<&str> = check.split("1").collect();
    let sum : u32 = check2[0].len() as u32;
    return if sum == complexity {
        println!("Checksum match !");
        let result: MD5HashCashOutput = MD5HashCashOutput {
            seed: i as u64,
            hashcode: result
        };
        tx.send(result).unwrap();
        true
    } else {
        println!("ERR");
        false
    }
}

fn main() {
    let input = MD5HashCashInput {
        complexity: 16,
        message: String::from("When i was child, i killed a boar at close range")
    };
    let test = HashCash::new(input);
    println!("{:?}", test.solve());
}
