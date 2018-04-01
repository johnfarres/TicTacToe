#[macro_use]
extern crate text_io;

fn check_win(board: &[[u8; 3]; 3]) -> bool {
    for i in 0..3 {
        if board[i][0] != 0 && board[i][0] == board[i][1] && board[i][0] == board[i][2] {
            return true;
        } else if board[0][i] != 0 && board[0][i] == board[1][i] && board[0][i] == board[2][i] {
            return true;
        }
    }

    if board[0][0] != 0 && board[0][0] == board[1][1] && board[0][0] == board[2][2] {
        return true;
    } else if board[0][2] != 0 && board[0][2] == board[1][1] && board[0][2] == board[2][0] {
        return true;
    }

    false
}

fn find_move(board: [[u8; 3]; 3], depth: u8) -> (u8, i32) {
    if depth % 2 == 0 {
        // Player's turn

        let mut score: i32 = -12;
        let mut current_move = 12;

        for i in 0..9 {
            if board[i / 3][i % 3] != 0 {
                continue;
            }

            let mut board_flag = board;
            board_flag[i / 3][i % 3] = 1;

            if check_win(&board_flag) {
                let score_flag: i32 = (depth as i32) - 10;

                if (score_flag < score) || current_move == 12 {
                    score = score_flag;
                    current_move = i;
                }
            } else if depth == 8 {
                let score_flag: i32 = 0;

                if (score_flag < score) || current_move == 12 {
                    score = score_flag;
                    current_move = i;
                }
            } else {
                let output: (u8, i32) = find_move(board_flag, depth + 1);

                if (output.1 < score) || current_move == 12 {
                    score = output.1;
                    current_move = i;
                }
            }
        }

        (current_move as u8, score)
    } else {
        // Engine's turn

        let mut score: i32 = -12;
        let mut current_move = 12;

        for i in 0..9 {
            if board[i / 3][i % 3] != 0 {
                continue;
            }

            let mut board_flag = board;
            board_flag[i / 3][i % 3] = 2;

            if check_win(&board_flag) {
                let score_flag: i32 = 10 - (depth as i32);

                if (score_flag > score) || current_move == 12 {
                    score = score_flag;
                    current_move = i;
                }
            } else if depth >= 8 {
                let score_flag: i32 = 0;

                if (score_flag > score) || current_move == 12 {
                    score = score_flag;
                    current_move = i;
                }
            } else {
                let output: (u8, i32) = find_move(board_flag, depth + 1);

                if (output.1 > score) || current_move == 12 {
                    score = output.1;
                    current_move = i;
                }
            }

            // print_board2(&board_flag);
        }

        (current_move as u8, score)
    }
    // if score > 0 {
    //     println!("{}", score);
    // }
}

fn print_board2(board2: &[[u8; 3]; 3]) {
    println!();
    println!(" -----------");
    println!(
        "| {} | {1} | {2} |",
        board2[0][0], board2[0][1], board2[0][2]
    );
    println!("------------");
    println!(
        "| {} | {1} | {2} |",
        board2[1][0], board2[1][1], board2[1][2]
    );
    println!("------------");
    println!(
        "| {} | {1} | {2} |",
        board2[2][0], board2[2][1], board2[2][2]
    );
    println!(" ----------- ");
    println!();
}

fn print_board(board2: &[[char; 3]; 3]) {
    println!();
    println!(" -----------");
    println!(
        "| {} | {1} | {2} |",
        board2[0][0], board2[0][1], board2[0][2]
    );
    println!("------------");
    println!(
        "| {} | {1} | {2} |",
        board2[1][0], board2[1][1], board2[1][2]
    );
    println!("------------");
    println!(
        "| {} | {1} | {2} |",
        board2[2][0], board2[2][1], board2[2][2]
    );
    println!(" ----------- ");
    println!();
}

fn main() {
    let mut board: [[u8; 3]; 3] = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];

    let mut board2: [[char; 3]; 3] = [[' ', ' ', ' '], [' ', ' ', ' '], [' ', ' ', ' ']];

    let mut turn: bool = false;
    let mut depth: u8 = 0;

    while !check_win(&board) {
        if turn {
            turn = false;
            let my_move = find_move(board, depth).0;
            println!("My move: {}", my_move + 1);
            board[(my_move as usize) / 3][(my_move as usize) % 3] = 2;
            board2[(my_move as usize) / 3][(my_move as usize) % 3] = 'O';
        } else {
            let mut input: usize;
            loop {
                println!("Enter your move!");
                input = read!();
                if board[(input - 1) / 3][(input - 1) % 3] != 0 {
                    println!("Error: not a valid position!");
                } else {
                    break;
                }
            }

            board[(input - 1) / 3][(input - 1) % 3] = 1;
            board2[(input - 1) / 3][(input - 1) % 3] = 'X';
            turn = true;
        }

        print_board(&board2);
        depth += 1;
    }

    match (depth - 1) % 2 {
        0 => println!("You are the winner!"),
        _ => println!("You lose! I win!"),
    }
}

#[test]
fn test_check_win() {
    println!("Test that checks the 'check_win()' function");

    let board1: [[u8; 3]; 3] = [[1, 2, 0], [0, 1, 2], [0, 0, 1]];

    match check_win(&board1) {
        true => println!("Test 1 succesful"),
        _ => println!("Test 1 unsuccesful"),
    }

    let board2: [[u8; 3]; 3] = [[1, 2, 0], [0, 2, 1], [0, 2, 1]];

    match check_win(&board2) {
        true => println!("Test 2 succesful"),
        _ => println!("Test 2 unsuccesful"),
    }
}
