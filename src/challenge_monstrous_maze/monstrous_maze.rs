pub(crate) fn string_to_matrix(s: &str) -> Vec<Vec<char>> {
    s.split('\n').map(|line| line.chars().collect()).collect()
}

pub(crate) fn find_character_position(v: &Vec<Vec<char>>, x: char) -> Option<(usize, usize)> {
    for (i, line) in v.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == x {
                return Some((i, j));
            }
        }
    }
    return None;
}

pub(crate) fn dijkstra(v: Vec<Vec<char>>, start: (usize, usize), end: (usize, usize), mut life: u8) -> Vec<(usize, usize)> {
    let mut dist: Vec<Vec<usize>> = vec![vec![2000; v[0].len()]; v.len()];
    let mut prev: Vec<Vec<(usize, usize)>> = vec![vec![(0, 0); v[0].len()]; v.len()];
    let mut visited: Vec<Vec<bool>> = vec![vec![false; v[0].len()]; v.len()];
    let mut queue: Vec<(usize, usize)> = Vec::new();
    queue.push(start);
    while !queue.is_empty() {
        let (i, j) = queue.pop().unwrap();

        if visited[i][j] {
            continue;
        }
        visited[i][j] = true;
        dist[i][j] = 0;
        if i == end.0 && j == end.1 {
            break;
        }

        for (i2, j2) in [(i as i32 - 1, j as i32), (i as i32 + 1, j as i32), (i as i32, j as i32 - 1), (i as i32, j as i32 + 1)].iter() {
            if *i2 < 0 || *i2 >= v.len() as i32 || *j2 < 0 || *j2 >= v[0].len() as i32 {
                continue;
            }
            let i2 = *i2 as usize;
            let j2 = *j2 as usize;

            if v[i2][j2] == '#' {
                continue;
            }

            if v[i2][j2] == 'M' && life == 1 {
                continue;
            }else if v[i2][j2] == 'M' {
                life -= 1;
            }

            if dist[i2][j2] > dist[i][j] + 1 {
                dist[i2][j2] = dist[i][j] + 1;
                prev[i2][j2] = (i, j);
                queue.push((i2.clone(), j2.clone()));
            }
        }
    }
    let mut path: Vec<(usize, usize)> = Vec::new();
    let mut current = end;
    while current != start {
        path.push(current);
        current = prev[current.0][ current.1];
    }
    path.push(start);
    path.reverse();

    return path;
}


pub(crate) fn path_to_direction(path: &Vec<(usize, usize)>) -> Vec<String> {
    let mut direction: Vec<String> = Vec::new();
    for i in 1..path.len() {
        if path[i].0 > path[i - 1].0 && path[i].1 == path[i - 1].1 {
            direction.push("v".to_string());
        } else if path[i].0 < path[i - 1].0 && path[i].1 == path[i - 1].1 {
            direction.push("^".to_string());
        }  else if path[i].0 == path[i - 1].0 && path[i].1 > path[i - 1].1 {
            direction.push(">".to_string());
        } else {
            direction.push("<".to_string());
        }
    }
    return direction;
}

#[test]
fn test_path_to_direction_to_bottom() {
    let path = vec![(0, 0), (1, 0)];
    let direction = path_to_direction(&path);
    assert_eq!(direction, vec!["v".to_string()]);
}

#[test]
fn test_path_to_direction_to_right() {
    let path = vec![(0, 0), (0, 1)];
    let direction = path_to_direction(&path);
    assert_eq!(direction, vec![">".to_string()]);
}

#[test]
fn test_path_to_direction_to_left() {
    let path = vec![(0, 1), (0, 0)];
    let direction = path_to_direction(&path);
    assert_eq!(direction, vec!["<".to_string()]);
}

#[test]
fn test_path_to_direction_to_top() {
    let path = vec![(1, 0), (0, 0)];
    let direction = path_to_direction(&path);
    assert_eq!(direction, vec!["^".to_string()]);
}

#[test]
fn test_complete_path_to_direction() {
    let path = vec![(0, 0), (1, 0), (2, 0), (2, 1), (3, 1), (3, 2), (3, 3), (4, 3)];
    let direction = path_to_direction(&path);
    assert_eq!(direction, vec!["v".to_string(), "v".to_string(), ">".to_string(), "v".to_string(), ">".to_string(), ">".to_string(), "v".to_string()] );
}

#[test]
fn test_complete_path_to_direction_of_complexe_example() {
    let path = vec![(2, 0), (2, 1), (1, 1), (0, 1), (0, 2), (0, 3), (1, 3), (2, 3), (3, 3), (3, 2), (4, 2), (5, 2), (5, 3), (6, 3), (7, 3), (8, 3), (8, 4), (9, 4), (9, 5), (9, 6), (8, 6)];
    let direction = path_to_direction(&path);
    assert_eq!(direction, vec![">".to_string(), "^".to_string(), "^".to_string(), ">".to_string(), ">".to_string(), "v".to_string(), "v".to_string(), "v".to_string(), "<".to_string(), "v".to_string(), "v".to_string(), ">".to_string(), "v".to_string(), "v".to_string(), "v".to_string(), ">".to_string(), "v".to_string(), ">".to_string(), ">".to_string(), "^".to_string()]  );
}



#[test]
fn test_dijkstra_with_two_case_move() {
    let v = string_to_matrix("I####\nX## #");
    /*
            "I####\n
             X## #\n"
    */
    let result_expected = Vec::from([(0, 0), (1, 0)]);
    let start = find_character_position(&v, 'I').unwrap();
    let end = find_character_position(&v, 'X').unwrap();
    let path = dijkstra(v.clone(), start, end, 1);
    assert_eq!(path, result_expected);
}

#[test]
fn test_dijkstra_with_three_case_move() {
    let v = string_to_matrix("I####\n ## #\nX# ##");
    let result_expected = Vec::from([(0, 0), (1, 0), (2, 0)]);
    let start = find_character_position(&v, 'I').unwrap();
    let end = find_character_position(&v, 'X').unwrap();
    let path = dijkstra(v.clone(), start, end, 1);
    assert_eq!(path, result_expected);
}

#[test]
fn test_dijkstra() {
    let v = string_to_matrix("I####\n ## #\n  # #\n#   #\n###X#");
    /*
           "I####\n
             ## #\n
              # #\n
            #   #\n
            ###X#"
    */

    let result_expected = Vec::from([(0, 0), (1, 0), (2, 0), (2, 1), (3, 1), (3, 2), (3, 3), (4, 3)]);
    let start = find_character_position(&v, 'I').unwrap();
    let end = find_character_position(&v, 'X').unwrap();
    let path = dijkstra(v.clone(), start, end, 1);
    //println!("je suis une pomme");
    //println!("{:?}", path);
    assert_eq!(path, result_expected);
}

#[test]
fn test_complex_dijkstra() {
    let v = string_to_matrix("I ###\nM # #\nX  ##");
    let v = string_to_matrix("      #\n  # # #\nI # # #\n##  # #\n##  # #\n##  # #\n##  ###\n##  ###\n##   #X\n####   ");
    /*
           "      #\n
              # # #\n
            I # # #\n
            ##  # #\n
            ##  # #\n
            ##  # #\n
            ##  ###\n
            ##  ###\n
            ##   #X\n
            ####   "
     */

    let result_expected = Vec::from([(2, 0), (2, 1), (1, 1), (0, 1), (0, 2), (0, 3), (1, 3), (2, 3), (3, 3), (3, 2), (4, 2), (5, 2), (5, 3), (6, 3), (7, 3), (8, 3), (8, 4), (9, 4), (9, 5), (9, 6), (8, 6)]);
    let start = find_character_position(&v, 'I').unwrap();
    let end = find_character_position(&v, 'X').unwrap();
    let path = dijkstra(v.clone(), start, end, 1);
    //println!("{:?}", path);
    assert_eq!(path, result_expected);
}

#[test]
fn test_simple_dijkstra_with_one_monster() {
    let v = string_to_matrix("I ###\nM # #\nX  ##");
    /*
        "I ###\n
         M # #\n
         X  ##"

    */

    let result_expected = Vec::from([(0, 0), (0, 1), (1, 1), (2, 1), (2, 0)]);
    let start = find_character_position(&v, 'I').unwrap();
    let end = find_character_position(&v, 'X').unwrap();
    let path = dijkstra(v.clone(), start, end, 1);
    //println!("{:?}", path);
    assert_eq!(path, result_expected);
}

#[test]
fn test_simple_dijkstra_with_two_monster_() {
    let v = string_to_matrix("M     #\n  # #  \nIM# #M \n##M MX ");
    /*"
            M     #\n
              # #  \n
            IM# #M \n
            ##M MX "
     */

    let result_expected = Vec::from([(2, 0), (2, 1), (1, 1), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (1, 5), (1, 6), (2, 6), (3, 6), (3, 5)]);
    let start = find_character_position(&v, 'I').unwrap();
    let end = find_character_position(&v, 'X').unwrap();
    let path = dijkstra(v.clone(), start, end, 2);
    //println!("{:?}", path);
    assert_eq!(path, result_expected);
}

#[test]
fn should_not_find_character_position() {
    let matrix = string_to_matrix("ABC\nDEF\nGHI");
    let result = find_character_position(&matrix, 'X');
    assert_eq!(result, None);
}

#[test]
fn should_find_character_position() {
    let matrix = string_to_matrix("ABC\nDXF\nGHI");
    let result = find_character_position(&matrix, 'X');
    assert_eq!(result, Some((1, 1)));
}

#[test]
fn should_find_character_position_at_last_position() {
    let matrix = string_to_matrix("ABC\nDEF\nGHI\n###\n###\n###\n##X");
    let result = find_character_position(&matrix, 'X');
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