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
        
        game_table_data[row][column] = match player_turn {
            Player::X => "X",
            Player::O => "O"
        };

        verify_win(&mut game_on, &game_table_data);

        player_turn = match player_turn {
            Player::X => Player::O,
            Player::O => Player::X
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
    x_hash_table: HashMap<usize, usize>,
    o_hash_table: HashMap<usize, usize>
}

impl GameTable {
    fn new() -> Self {
        GameTable {
            x_hash_table: HashMap::new(),
            o_hash_table: HashMap::new()
        }
    }

    fn build(&mut self, game_table_data: &Vec<Vec<&str>>) {
        for i in 0..game_table_data.iter().collect::<Vec<_>>().len() {
            for j in 0..game_table_data[i].iter().collect::<Vec<_>>().len() {
                if game_table_data[i][j] == "X" {
                    self.x_hash_table.insert(i, j);
                }
                
                if game_table_data[i][j] == "O" {
                    self.o_hash_table.insert(i, j);
                }
            }
        }
    }
}

fn verify_win(game_on: &mut bool, game_table_data: &Vec<Vec<&str>>) {
    let mut game_table = GameTable::new();

    game_table.build(&game_table_data);

    //verifiy

    *game_on = false;
}
