use crate::challenge::Challenge;
use crate::challenge_message::{MonstrousMazeInput, MonstrousMazeOutput};
use crate::challenge_monstrous_maze::monstrous_maze::{dijkstra, path_to_direction};
use crate::challenge_monstrous_maze::utils::monstrous_maze_utils::{find_character_position_matrix, string_to_matrix};

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
        let start = find_character_position_matrix(&grid_game, 'I').unwrap();
        let end = find_character_position_matrix(&grid_game, 'X').unwrap();
        let path = dijkstra(grid_game, start, end, self.input.endurance.clone());
        let result = path_to_direction(&path);

        return MonstrousMazeOutput {
            path: result.join("").to_string(),
        };
    }

    fn verify(&self, _: Self::Output) -> bool {
        todo!()
    }
}