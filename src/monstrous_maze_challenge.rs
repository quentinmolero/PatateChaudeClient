use crate::challenge::Challenge;
use crate::challenge_message::{MonstrousMazeInput, MonstrousMazeOutput};
use crate::monstrous_maze::{dijkstra, find_character_position, path_to_direction, string_to_matrix};

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

        let grid_game = string_to_matrix(self.input.grid.as_str());
        let start = find_character_position(&grid_game, 'I').unwrap();
        let end = find_character_position(&grid_game, 'X').unwrap();
        let path = dijkstra(grid_game, start, end);
        let result = path_to_direction(&path);

        return MonstrousMazeOutput {
            path: result.join("").to_string(),
        };
    }

    fn verify(&self, _: Self::Output) -> bool {
        todo!()
    }
}