use std::fmt::{Display, Formatter, Result as fmtResult};
use std::io;
use rand::prelude::*;
use std::cmp::PartialEq;


// 3 states of a postion is represented by and enum called Status
// if status Nought or Cross, position is occupied otherwise position is empty
#[derive(Copy, Clone, PartialOrd, PartialEq)]
enum Status {CROSS, NOUGHT, NUMBER(i32)}
impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        match self {
            Self::CROSS => write!(f, "X"),
            Self::NOUGHT => write!(f, "O"),
            Self::NUMBER(value) => write!(f, "{}", value),
        }
    }
}

// there are 2 players human and computer represented by struct Players
struct Players {
    Human: Status,
    Computer: Status,
}

impl Players{
    fn new() -> Self {
        // this function creates a player struct and assigns nought or cross randomly to players
        if rand::random() {
            Players{
                Human: Status::NOUGHT,
                Computer: Status::CROSS,
            }
        } else {
            Players{
                Human: Status::CROSS,
                Computer: Status::NOUGHT,
            }
        }
    }
}

// to represent and alternate turns, we have an enum called CurrentPlayer
// that can take value of Human or Computer
#[derive(PartialOrd, PartialEq)]
enum CurrentPlayer {Human = 1, Computer=2 }

fn print_instructions() {
    println!(" Instructions ");
    println!("1. Players alternate placing Xs and Os on the board until either \
    player has three in a row, horizontally, vertically, or diagonally or until\
     all squares on the grid are filled ");
    println!("2. If a player is able to draw three Xs or three Os in a row, \
    then that player wins ");
    println!("3. If all squares are filled and neither player has made a complete \
    row of Xs or Os, then the game is a draw ");
    println!("Press enter to continue...");
    let mut sss = String::new();
    // these are the ignored entered pressed by the user
    io::stdin().read_line(&mut sss);
    io::stdin().read_line(&mut sss);
}

fn create_board() -> Vec<Vec<Status>> {
    // it creates 3x3 board and position is represented by Status
    let mut vec = vec![];
    for i in 0..3 {
        let mut temp = vec![];
        for j in 1..4 {
            temp.push(Status::NUMBER((i*3+j) as i32));
        }
        vec.push(temp);
    }
    vec
}

fn print_board(board: &Vec<Vec<Status>>) {
    println!( " Tic Tac Toe Board " );
    for row in board {
        for ent in row {
            print!(" {} |", ent)
        }
        println!();
        for _ in row {
            print!("____");
        }
        println!();
    }
}

fn win(board: &Vec<Vec<Status>>, players: &Players) -> String {
    // it checks in rows, columns and diagonals if there is a winner or not
    let Human = "Human".to_string();
    let Computer = "Computer".to_string();
    // checking in rows if there is a winner
    for vec in board {
        if vec[0]==vec[1] && vec[0]==vec[2] {
            return if vec[0] == players.Human {
                Human
            } else {
                Computer
            }
        }
    }
    // checking in columns if there is a winner
    for i in 0..3 {
        if board[0][i]==board[1][i] && board[0][i]==board[2][i] {
            return if board[0][i] == players.Human {
                Human
            } else {
                Computer
            }
        }
    }
    // checking in diagonals if theres is a winner
    if board[0][0] == board[1][1] && board[0][0] == board[2][2] {
        return if board[1][1] == players.Human {
            Human
        } else {
            Computer
        }
    }
    if board[0][2] == board[1][1] && board[2][0] == board[1][1] {
        return if board[1][1] == players.Human {
            Human
        } else {
            Computer
        }
    }
    return String::new()
}


fn get_computer_move(availability_vec: &mut Vec<i32>) -> i32 {
    // this function takes an availability vec, shuffle it randomly
    // and returns it's first index as the computers move
    // there could be multiple strategies for computer's move like
    // random number can be generated in the range of len of vec
    // and returned that element in the vec
    availability_vec.shuffle(&mut thread_rng());
    availability_vec.get(0).unwrap().clone()
}

fn get_move(availability_vec: &Vec<i32>, current_player: &CurrentPlayer) -> i32 {
    // this function checks if it's computers move or human's move
    // if it's computers turn, it simply call get_computer_move
    // otherwise it asks user for a valid move until he enters a valid move
    let mut player_move = 0;
    if current_player == &CurrentPlayer::Human {
        while player_move > 9 || player_move < 1 || !availability_vec.contains(&player_move){
            println!("Enter a valid move from 1 to 9 : ");
            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();
            player_move = line.trim().parse().unwrap_or(0);
            if player_move==0 {
                println!("You have selected an invalid move.")
            }else {
                println!("you selsected {}", player_move);
            }
        }
    } else {
        println!("It's computer's turn...");
        player_move = get_computer_move(&mut availability_vec.clone());
        println!("Computer selected {}", player_move);
    }
    player_move
}

fn play_move(board: &mut Vec<Vec<Status>>, availability_vec: &mut Vec<i32>,
             players:&Players, current_player: &mut CurrentPlayer, player_move: i32) {
    // this function makes the actual changes. it takes most of the parameters as mutale
    // it removes the selected element from availability vector and
    // mark the position as completed in the board
    // it also alternates the turns of players

    // removing selected element from availability vec
    let index = availability_vec.iter().position(|&r| r==player_move).unwrap();
    availability_vec.remove(index);

    // marking selected element as occupied in the board
    let row = (player_move-1)/3;
    let col = (player_move-1)%3;
    board[row as usize][col as usize] = match current_player {
        CurrentPlayer::Human => players.Human.clone(),
        CurrentPlayer::Computer => players.Computer.clone(),
    };

    // alternating turns of players
    if current_player == &mut CurrentPlayer::Human {
        *current_player = CurrentPlayer::Computer;
    } else {
        *current_player = CurrentPlayer::Human;
    }

}

fn play_tic_tac_toe() {
    let mut board = create_board();
    let mut availability_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let players = Players::new();
    let mut current_player = CurrentPlayer::Computer;
    if players.Human == Status::CROSS { current_player = CurrentPlayer::Human};
    let mut player_move = 0;
    let mut winner = String::new();
    println!("Welcome to Tic Tac Toe");
    print_instructions();
    println!("Human takes {} and Computer takes {}", &players.Human, &players.Computer);
    while !availability_vec.is_empty() {
        print_board(&board);
        player_move = get_move(&availability_vec, &current_player);
        play_move(&mut board, &mut availability_vec, &players, &mut current_player, player_move);
        winner = win(&board, &players);
        if !winner.is_empty() {
            break;
        }
    }
    print_board(&board);
    if winner.is_empty() {
        println!("Game Over \n Game ended in a draw");
    } else {
        println!("Game Over \n Winner is : {}", winner);
    }
}

fn main() {
    let mut response = String::new();
    loop {
        play_tic_tac_toe();
        println!("Do you want to play again(Y/N)");
        io::stdin().read_line(&mut response).unwrap();
        if response.trim().to_lowercase().eq(&"y".to_string()){
            continue
        }
        println!("Good Bye");
        break;
    }
}
