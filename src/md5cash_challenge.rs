use std::sync::{Arc, mpsc};
use std::sync::atomic::{AtomicBool, AtomicU64};
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
                    //let mut check = format!("{:0128b}", binary);

                    if verify_hashcash(complexity, result.clone()) {
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

        return rx.recv().unwrap();
    }

    fn verify(&self, _: Self::Output) -> bool {
        todo!()
    }
}

fn verify_hashcash(complexity : u32, hashcode: String) -> bool {
    let binary = u128::from_str_radix(&*hashcode, 16).unwrap();
    if binary.leading_zeros() == complexity {
        return true;
    }

    return false;
}

trait CommandOutput {
    fn get_command_output() -> Result<(), ()>;
}


#[cfg(test)]
mod tests {
    use crate::challenge::Challenge;
    use crate::challenge_message::{MD5HashCashInput, MD5HashCashOutput};
    use crate::md5cash_challenge::{HashCash, verify_hashcash};

    fn solve_test(complexity : u32, message: String ) -> MD5HashCashOutput {
        let input : MD5HashCashInput = MD5HashCashInput {
            complexity,
            message
        };

        return HashCash::new(input).solve();

    }

    #[test]
    fn simple_hashcash() {
        let complexity : u32 = 5;
        let message = "Hello world".to_string();

        let output = solve_test(complexity, message);

        assert!(verify_hashcash(complexity, output.hashcode))
    }

    #[test]
    fn long_hashcash() {
        let complexity : u32 = 5;
        let message = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla fermentum porttitor ex, at dapibus neque. Suspendisse a mauris et lectus vestibulum efficitur.".to_string();

        let output = solve_test(complexity, message);

        assert!(verify_hashcash(complexity, output.hashcode))
    }

    #[test]
    fn simple_hashcash_complex() {
        let complexity : u32 = 20;
        let message = "Hello world".to_string();

        let output = solve_test(complexity, message);

        assert!(verify_hashcash(complexity, output.hashcode))
    }

    #[test]
    fn long_hashcash_complex() {
        let complexity : u32 = 20;
        let message = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla fermentum porttitor ex, at dapibus neque. Suspendisse a mauris et lectus vestibulum efficitur.".to_string();

        let output = solve_test(complexity, message);

        assert!(verify_hashcash(complexity, output.hashcode))
    }
}
