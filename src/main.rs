use std::collections::HashMap;
use std::io;

const AI_PLAYER_MARKER: PlayerState = PlayerState::X;

fn main() {
    let mut game_board = vec![[SquareState::Empty, SquareState::Empty, SquareState::Empty],
                              [SquareState::Empty, SquareState::Empty, SquareState::Empty],
                              [SquareState::Empty, SquareState::Empty, SquareState::Empty]];

    let initial_player = AI_PLAYER_MARKER.get_other_player();

    let mut round_count = 0;

    loop {
        if round_count > 0 || initial_player == AI_PLAYER_MARKER {
            let mut temp_board = game_board.clone();

            let mut highest_win_percentage = 0.0;
            let mut lowest_loss_percentage = 100.0;
            let mut lowest_outcomes = 0;


            for row in 0..3 {
                for column in 0..3 {
                    if temp_board[row][column] != SquareState::Empty {
                        continue;
                    }
                    temp_board[row][column] = AI_PLAYER_MARKER.convert_to_square_state();

                    let outcomes = find_outcomes(&temp_board, AI_PLAYER_MARKER.get_other_player());
                    let total_outcomes: i64 = outcomes.values().sum();

                    if total_outcomes == 0 {
                        if analyze_board(&temp_board, AI_PLAYER_MARKER) != Outcome::Unfinished {
                            game_board = temp_board.clone();
                            break;
                        } else {
                            continue;
                        }
                    }

                    let wins = *outcomes.get(&Outcome::Win).unwrap();
                    let mut win_percentage: f64 = (wins as f64) / (total_outcomes as f64) * 100.0;

                    let draws = *outcomes.get(&Outcome::Draw).unwrap();
                    let draw_percentage: f64 = (draws as f64) / (total_outcomes as f64) * 100.0;

                    let losses = *outcomes.get(&Outcome::Loss).unwrap();
                    let loss_percentage: f64 = (losses as f64) / (total_outcomes as f64) * 100.0;

                    win_percentage += draw_percentage * 0.25;

                    if win_percentage > highest_win_percentage {
                        highest_win_percentage = win_percentage;
                        lowest_outcomes = total_outcomes;
                        lowest_loss_percentage = loss_percentage;
                        game_board = temp_board.clone();
                    } else if win_percentage == highest_win_percentage {
                        if loss_percentage < lowest_loss_percentage {
                            lowest_loss_percentage = loss_percentage;
                            lowest_outcomes = total_outcomes;
                            game_board = temp_board.clone();
                        }
                    } else if win_percentage == highest_win_percentage &&
                        loss_percentage == lowest_loss_percentage {
                        if total_outcomes < lowest_outcomes {
                            lowest_outcomes = total_outcomes;
                            game_board = temp_board.clone();
                        }
                    }

                    temp_board[row][column] = SquareState::Empty;
                }
            }
            println!("AI's chance of winning: {:.2}%\n", highest_win_percentage);
            println!();
        }
        print_board(&game_board);

        let current_game_state = analyze_board(&game_board, AI_PLAYER_MARKER.get_other_player());
        if current_game_state != Outcome::Unfinished {
            match current_game_state {
                Outcome::Win => println!("Congratulations, you won!"),
                Outcome::Loss => println!("I'm sorry, you lost! :("),
                Outcome::Draw => println!("It was a draw, so you didn't win or lose!"),
                _ => println!("Game not finished yet"),
            }
            return;
        }

        loop {
            println!("Where would you like to play?");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Couldn't read from stdin");

            let input_chars: Vec<char> = input.trim().chars().collect();

            let input_error_message = "\nERROR: Please input a coordinate position, ex. - A2\n";

            if input_chars.len() != 2 {
                println!("{}", input_error_message);
                continue;
            }

            let row: char = input_chars[0];
            let column: char = input_chars[1];

            let row = match row {
                'A' => 0,
                'B' => 1,
                'C' => 2,
                _ => {
                    println!("{}", input_error_message);
                    continue;
                }
            };

            let column = match column {
                '1' => 0,
                '2' => 1,
                '3' => 2,
                _ => {
                    println!("{}", input_error_message);
                    continue;
                }
            };

            if game_board[row][column] != SquareState::Empty {
                println!("\nERROR: Please choose an empty square\n");
                continue;
            }

            game_board[row][column] = AI_PLAYER_MARKER.get_other_player().convert_to_square_state();
            break;
        }
        round_count += 1;
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Outcome {
    Win,
    Loss,
    Draw,
    Unfinished,
}


#[derive(Debug, PartialEq, Copy, Clone)]
enum SquareState {
    X,
    O,
    Empty,
}

impl SquareState {
    fn convert_to_player_state(&self) -> PlayerState {
        match *self {
            SquareState::X => PlayerState::X,
            SquareState::O => PlayerState::O,
            SquareState::Empty => {
                panic!("Can't convert SquareState::Empty to PlayerState");
            }
        }
    }
}


#[derive(Debug, PartialEq, Copy, Clone)]
enum PlayerState {
    X,
    O,
}

impl PlayerState {
    fn convert_to_square_state(&self) -> SquareState {
        match *self {
            PlayerState::X => SquareState::X,
            PlayerState::O => SquareState::O,
        }
    }

    fn get_other_player(&self) -> PlayerState {
        match *self {
            PlayerState::X => PlayerState::O,
            PlayerState::O => PlayerState::X,
        }
    }
}

fn analyze_board(board: &Vec<[SquareState; 3]>, current_player: PlayerState) -> Outcome {
    let mut streak_value;
    let mut streak_count;

    /*
     * Check rows for outcome
     */

    for row in board.iter() {
        streak_value = SquareState::Empty;
        streak_count = 0;

        for &value in row.iter() {
            if value == SquareState::Empty {
                break;
            }

            if streak_value == SquareState::Empty {
                streak_value = value;
            }

            if value != streak_value {
                break;
            }

            streak_count += 1;

            if streak_count == 3 {
                if current_player == streak_value.convert_to_player_state() {
                    return Outcome::Win;
                } else {
                    return Outcome::Loss;
                }
            }
        }
    }

    /*
     * Check columns for outcome
     */

    for column in 0..3 {
        streak_value = SquareState::Empty;
        streak_count = 0;

        for row in board.iter() {
            let value = row[column];

            if value == SquareState::Empty {
                break;
            }

            if streak_value == SquareState::Empty {
                streak_value = value;
            }

            if value != streak_value {
                break;
            }

            streak_count += 1;

            if streak_count == 3 {
                if current_player == streak_value.convert_to_player_state() {
                    return Outcome::Win;
                } else {
                    return Outcome::Loss;
                }
            }
        }
    }

    /*
     * Check diagonal (top-left to bottom-right) for outcome
     */

    streak_value = SquareState::Empty;
    streak_count = 0;

    for index in 0..3 {
        let value = board[index][index];

        if value == SquareState::Empty {
            break;
        }

        if streak_value == SquareState::Empty {
            streak_value = value;
        }

        if value != streak_value {
            break;
        }

        streak_count += 1;

        if streak_count == 3 {
            if current_player == streak_value.convert_to_player_state() {
                return Outcome::Win;
            } else {
                return Outcome::Loss;
            }
        }
    }

    /*
     * Check diagonal (bottom-left to top-right) for outcome
     */

    streak_value = SquareState::Empty;
    streak_count = 0;

    for index in 0..3 {
        let value = board[index][2 - index];

        if value == SquareState::Empty {
            break;
        }

        if streak_value == SquareState::Empty {
            streak_value = value;
        }

        if value != streak_value {
            break;
        }

        streak_count += 1;

        if streak_count == 3 {
            if current_player == streak_value.convert_to_player_state() {
                return Outcome::Win;
            } else {
                return Outcome::Loss;
            }
        }
    }

    /*
     * Check for empty values
     */

    for row in board.iter() {
        for &value in row.iter() {
            if value == SquareState::Empty {
                return Outcome::Unfinished;
            }
        }
    }

    // If this is reached then there is a Draw
    Outcome::Draw
}


fn find_outcomes(board: &Vec<[SquareState; 3]>,
                 current_player: PlayerState)
                 -> HashMap<Outcome, i64> {
    let mut outcomes = HashMap::new();
    outcomes.insert(Outcome::Win, 0);
    outcomes.insert(Outcome::Loss, 0);
    outcomes.insert(Outcome::Draw, 0);

    let mut temp_board = board.clone();

    let outcome = analyze_board(&temp_board, AI_PLAYER_MARKER);
    if outcome != Outcome::Unfinished {
        *outcomes.entry(outcome).or_insert(0) += 1;
        return outcomes;
    }

    for row in 0..3 {
        for column in 0..3 {
            if temp_board[row][column] != SquareState::Empty {
                continue;
            }
            temp_board[row][column] = current_player.convert_to_square_state();
            let outcome = analyze_board(&temp_board, AI_PLAYER_MARKER);
            if outcome != Outcome::Unfinished {
                *outcomes.entry(outcome).or_insert(0) += 1;
                continue;
            }

            let nested_outcomes = find_outcomes(&temp_board, current_player.get_other_player());
            for (nested_outcome, value) in &nested_outcomes {
                *outcomes.entry(nested_outcome.clone()).or_insert(0) += *value;
            }

            temp_board[row][column] = SquareState::Empty;
        }
    }

    return outcomes;
}


fn print_board(board: &Vec<[SquareState; 3]>) {
    println!("  | 1 | 2 | 3 |");
    println!("---------------");

    let letters = ['A', 'B', 'C'];
    let mut letter_index = 0;

    for row in board.iter() {
        print!("{} |", letters[letter_index]);
        letter_index += 1;

        for &value in row.iter() {
            if value == SquareState::Empty {
                print!(" - |");
                continue;
            }

            print!(" {:?} |", value);
        }
        println!("\n---------------");
    }
    println!()
}
