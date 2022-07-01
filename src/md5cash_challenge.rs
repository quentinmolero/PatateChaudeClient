use std::ptr::{eq, hash};
use std::process::{Command, Stdio};
use std::str::from_utf8;
use std::sync::{Arc, LockResult, mpsc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
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
        let (tx, rx): (Sender<MD5HashCashOutput>, Receiver<MD5HashCashOutput>) = mpsc::channel();
        let mut i : u64 = 0;

        let mut valid = Arc::new(AtomicBool::new(false));

        for th in 0..threads {
            let valid = valid.clone();

            let tx = tx.clone();
            let complexity = self.input.complexity.clone();
            let message = self.input.message.clone();
            thread::spawn(move || {
                loop {
                    if valid.load(Ordering::Relaxed) {
                        break;
                    }

                    let hash = format!("{:016X}", i) + &message;

                    let result = format!("{:x}", md5::compute(hash));

                    let binary   = u128::from_str_radix(&*result, 16).unwrap();
                    let mut check = format!("{:0128b}", binary);
                    let check2 : Vec<&str> = check.split("1").collect();
                    let sum : u32 = check2[0].len() as u32;
                    if sum == complexity {
                        let result : MD5HashCashOutput = MD5HashCashOutput {
                            seed: i as u64,
                            hashcode : result
                        };
                        valid.store(true, Ordering::Relaxed);
                        tx.send(result).unwrap();

                    }
                    i += 1;

                }

            });

        }

        for message in rx.iter().take(threads) {
            return message;
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