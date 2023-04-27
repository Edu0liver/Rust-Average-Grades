use std::{io, collections::HashMap};
use rand::Rng;

enum Player {
    X,
    O
}

fn main() {
    let mut game_on = true;
    let mut player_turn = starter_player();
    
    let mut game_table_data: Vec<Vec<&str>> = Vec::from([
        Vec::from(["_", "|", "_", "|", "_"]),
        Vec::from(["_", "|", "_", "|", "_"]),
        Vec::from([" ", "|", " ", "|", " "]),
    ]);

    while game_on {

        println!("{}\n", build_current_game(&game_table_data));

        match player_turn {
            Player::X => {
                println!("Player X turn");
                println!("Tip the position you want to play the X (row column): ");
            },
            Player::O => {
                println!("Player O turn");
                println!("Tip the position you want to play the O (row column): ");
            }
        };

        let (row, column) = get_input();
        
        if game_table_data[row][column] == "X" || game_table_data[row][column] == "O" {
            println!("Tente outra posição!\n");
            continue
        };

        game_table_data[row][column] = match player_turn {
            Player::X => "X",
            Player::O => "O"
        };

        verify_win(&mut game_on, &game_table_data);

        player_turn = match player_turn {
            Player::X => Player::X,
            Player::O => Player::O
        };
    }
}

fn build_current_game(game_table_data: &Vec<Vec<&str>>) -> String {

    let mut game_table = String::new();

    for i in game_table_data[0].iter().collect::<Vec<_>>() {
        game_table.push_str(i);
    }
    
    game_table.push_str("\n");

    for i in game_table_data[1].iter().collect::<Vec<_>>() {
        game_table.push_str(i);
    }

    game_table.push_str("\n");

    for i in game_table_data[2].iter().collect::<Vec<_>>() {
        game_table.push_str(i);
    }
    
    game_table
}

fn starter_player() -> Player {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0..2);

    if random_number == 0 {
        Player::X
    } else {
        Player::O
    }
}

fn get_input() -> (usize, usize) {

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .unwrap();

    let input_data: Vec<&str> = input.split(' ').collect();

    let row = input_data[0].trim().parse::<usize>().unwrap() - 1;

    let column_calc = (2 as usize).pow(input_data[1].trim().parse::<usize>().unwrap() as u32 - 1);

    let column = if column_calc != 1 {
        column_calc
    } else {
        0
    };

    (row, column)
}

struct GameTable {
    x_positions: Vec<(usize, usize)>,
    o_positions: Vec<(usize, usize)>
}

impl GameTable {
    fn new() -> Self {
        GameTable {
            x_positions: Vec::new(),
            o_positions: Vec::new()
        }
    }

    fn build(&mut self, game_table_data: &Vec<Vec<&str>>) {
        for i in 0..game_table_data.iter().collect::<Vec<_>>().len() {
            for j in 0..game_table_data[i].iter().collect::<Vec<_>>().len() {
                match game_table_data[i][j] {
                    "X" => self.x_positions.push((i, j)),
                    "O" => self.o_positions.push((i, j)),
                    _ => continue
                }
            }
        }
    }

    fn verify_game_win(&self, game_table_data: &Vec<Vec<&str>>) -> Option<String> {
        
        if has_three_equal_numbers(&self.x_positions) {
            return Some(String::from("X"))
        }

        if has_three_equal_numbers(&self.o_positions) {
            return Some(String::from("O"))
        }

        if let Some(winner) = get_diagonal(game_table_data) {
            return Some(String::from(winner))
        }

        if let Some(winner) = get_reverse_diagonal(game_table_data) {
            return Some(String::from(winner))
        }
        
        None
    }

}

fn verify_win(game_on: &mut bool, game_table_data: &Vec<Vec<&str>>) -> Option<String> {
    let mut game_table = GameTable::new();

    game_table.build(&game_table_data);

    if let Some(winner) = game_table.verify_game_win(game_table_data) {
        *game_on = false;
        
        println!("{}\n", build_current_game(&game_table_data));

        println!("{} is the winner!", winner)
    }

    None

}

fn has_three_equal_numbers(vec: &Vec<(usize, usize)>) -> bool {

    let mut row_count = HashMap::new();
    let mut column_count = HashMap::new();
    
    for position in vec.iter() {
        row_count.entry(position.0)
            .and_modify(|e| *e += 1)
            .or_insert(1);

        column_count.entry(position.1)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
    
    match row_count.into_values().collect::<Vec<usize>>().iter().find(|&&x| x == 3) {
        Some(_) => true,
        None => {
            match column_count.into_values().collect::<Vec<usize>>().iter().find(|&&x| x == 3) {
                Some(_) => true,
                None => false
            }
        }
    }

}

fn get_diagonal(matrix: &Vec<Vec<&str>>) -> Option<String> {
    let mut diagonal = Vec::new();
    
    for i in 0..matrix.len() {
        diagonal.push(matrix[i][i]);
    }

    if diagonal.iter().filter(|e| e.contains("X")).collect::<Vec<_>>().len() >= 3 {
        return Some(String::from("X"))
    }

    if diagonal.iter().filter(|e| e.contains("O")).collect::<Vec<_>>().len() >= 3 {
        return Some(String::from("O"))
    }
    
    None
}

fn get_reverse_diagonal(matrix: &Vec<Vec<&str>>) -> Option<String> {
    let mut diagonal = Vec::new();
    
    for i in 0..matrix.len() {
        diagonal.push(matrix[matrix.len()-1-i][i]);
    }

    if diagonal.iter().filter(|e| e.contains("X")).collect::<Vec<_>>().len() >= 3 {
        return Some(String::from("X"))
    }

    if diagonal.iter().filter(|e| e.contains("O")).collect::<Vec<_>>().len() >= 3 {
        return Some(String::from("O"))
    }
    
    None
}
