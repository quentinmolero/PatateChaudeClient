pub(crate) fn string_to_matrix(s: &str) -> Vec<Vec<char>> {
    s.split('\n').map(|line| line.chars().collect()).collect()
}

pub(crate) fn find_character_position_matrix(v: &Vec<Vec<char>>, x: char) -> Option<(usize, usize)> {
    for (i, line) in v.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == x {
                return Some((i, j));
            }
        }
    }
    return None;
}

#[test]
fn should_not_find_character_position() {
    let matrix = string_to_matrix("ABC\nDEF\nGHI");
    let result = find_character_position_matrix(&matrix, 'X');
    assert_eq!(result, None);
}

#[test]
fn should_find_character_position() {
    let matrix = string_to_matrix("ABC\nDXF\nGHI");
    let result = find_character_position_matrix(&matrix, 'X');
    assert_eq!(result, Some((1, 1)));
}

#[test]
fn should_find_character_position_at_last_position() {
    let matrix = string_to_matrix("ABC\nDEF\nGHI\n###\n###\n###\n##X");
    let result = find_character_position_matrix(&matrix, 'X');
    assert_eq!(result, Some((6, 2)));
}


#[test]
fn should_return_matrix_with_one_line() {
    let result = vec![vec!['#', '#', '#']];
    let string = "###";

    let matrix = string_to_matrix(string);

    assert_eq!(result, matrix);
}

#[test]
fn should_return_matrix_with_two_lines() {
    let result = vec![vec!['#', 'I', '#'], vec!['#', '#', '#']];
    let string = "#I#\n###";

    let matrix = string_to_matrix(string);

    assert_eq!(result, matrix);
}

#[test]
fn should_return_matrix_with_three_lines() {
    let result = vec![vec!['#', '#', 'I'], vec!['#', 'M', '#'], vec!['#', '#', '#']];
    let string = "##I\n#M#\n###";

    let matrix = string_to_matrix(string);

    assert_eq!(result, matrix);
}
