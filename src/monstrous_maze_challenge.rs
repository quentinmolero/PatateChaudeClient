use crate::challenge::Challenge;
use crate::challenge_message::{MonstrousMazeInput, MonstrousMazeOutput};

pub(crate) struct Monstrous {
    input: MonstrousMazeInput,
}

impl Challenge for Monstrous {
    type Input = MonstrousMazeInput;
    type Output = MonstrousMazeOutput;

    fn name() -> String {
        String::from("Monstrous")
    }

    fn new(input: Self::Input) -> Monstrous {
        Monstrous {
            input,
        }
    }

    fn solve(&self) -> MonstrousMazeOutput {

        return MonstrousMazeOutput {
            path: "".to_string(),
        };
    }

    fn verify(&self, _: Self::Output) -> bool {
        todo!()
    }
}