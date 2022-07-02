use std::sync::{Arc, mpsc};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::atomic::Ordering::Relaxed;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
// use std::time::Instant;
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
        let i  = Arc::new(AtomicU64::new(0));

        let valid = Arc::new(AtomicBool::new(false));

        for _ in 0..threads {
            // let a = Instant::now();

            let i = i.clone();
            let valid = valid.clone();

            let tx = tx.clone();
            let complexity = self.input.complexity.clone();
            let message = self.input.message.clone();
            thread::spawn(move || {
                loop {
                    if valid.load(Relaxed) {
                        break;
                    }

                    let seed = i.load(Relaxed);

                    let hash = format!("{:016X}", seed) + &message;

                    let result = format!("{:016X}", md5::compute(hash));

                    let binary = u128::from_str_radix(&*result, 16).unwrap();
                    //let mut check = format!("{:0128b}", binary);


                    if binary.leading_zeros() == complexity {
                        let result : MD5HashCashOutput = MD5HashCashOutput {
                            seed,
                            hashcode : result
                        };
                        valid.store(true, Relaxed);
                        tx.send(result).unwrap();

                    }
                    i.fetch_add(1, Relaxed);

                }

            });


        }

        let machin = rx.recv().unwrap();

        return machin;

    }

    fn verify(&self, _: Self::Output) -> bool {
        todo!()
    }
}

trait CommandOutput {
    fn get_command_output() -> Result<(), ()>;
}
