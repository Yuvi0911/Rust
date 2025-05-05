use std::io;

const PLAYER_X: char = 'X';
const PLAYER_O: char = 'O';

const BOARD_SIZE: usize = 3;

// 2D array => 3X3
// [char; BOARD_SIZE] => no of columns
// BOARD_SIZE => no of rows
// [[char; BOARD_SIZE]; BOARD_SIZE] => humne ye 2d array bnayi h aur ushe Board k andar assign kr diya h. Ab hume is 2d array vale syntax ki jagah Board use kr skte h.
type Board = [[char; BOARD_SIZE]; BOARD_SIZE];

fn initialize_board() -> Board {
    return [[' ';BOARD_SIZE]; BOARD_SIZE];
}

fn print_board(board: &Board) {
    for row in board{
        for cell in row{
            print!("{} ", cell);
        }
        println!()
    }
}

fn get_player_move(current_player: char, board: &Board) -> (usize, usize) {
    loop{
        println!("Player {} input (row, col):", current_player);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        println!("input = {}", input);

        /*
            humne input me string li h "0 1".
            1) hum is string ko split krege space k basics pr. => "0", "1"
            2) "0", "1" ko integer me convert krege flat_map ki help se.
            3) collect method ki help se coordinate vector me assign kr dege.
        */
        let coordinates:Vec<usize> = input
            .split_whitespace()
            .flat_map(str::parse::<usize>)
            .collect();

            //  user ne jo input di h vo hamesa 2 value honi chaiye.
            if coordinates.len()==2{
                let (row,col)=(coordinates[0],coordinates[1]);
                // jo 2 value h row aur col ki vo hamesha board k andar exist krni chaiye aaur board me vo space empty hona chaiye.
                if row < BOARD_SIZE && col < BOARD_SIZE && board[row][col]==' '{
                    // user ne jo entry di h ushe return krdege. Aur us entry pr X ya O fill kr dege.
                    return (row,col);
                }
            }
            // yadi if vali cond. match nhi kregi toh user ne wrong input di h, isliye ye loop dobara execute hoga.
            println!("Invalid Input");
        }
}

fn check_winner(current_player: char, board: &Board)->bool{
    // row => yadi puri row me current_playe h toh current_player jit jayga.
    for row in 0..BOARD_SIZE{
        if board[row][0] == current_player && board[row][1]==current_player && board[row][2] == current_player{
            return true;
        }
    }
    // col => yadi puri column me current_playe h toh current_player jit jayga.
    for col in 0..BOARD_SIZE {
        if board[0][col] == current_player && board[1][col] == current_player && board[2][col] == current_player{
            return true;
        }
    }

    // diagonal => yadi puri diagonal me current_player h toh current_player jit jayga.
    if (board[0][0] == current_player && board[1][1] == current_player && board[2][2] == current_player) || (board[0][2] == current_player && board[1][1] == current_player || board[2][0] == current_player){
        return true;
    }

    // yadi koi bhi cond match nhi hui toh false return kr dege ki current_player abhi nhi jita.
    return false;
}

fn check_draw(board:&Board)->bool{
    for row in board{
        for cell in row{
            // yadi koi empty cell nhi bachi hui toh game abhi draw nhi hua h.
            // hum cell k content ko check kr rhe h isliye de-refrence operator use kr rhe h.
            if *cell==' '{
                return false;
            }
        }
    }
    // yadi koi bhi empty cell nhi bachi hui toh game draw ho gya h.
    return true;
}


fn play_game() {
    // jab bhi user game start krega toh hum board k liye ek 2d array initialize krege. aur hum ushe board me assign krdege aur board ko mutable bna dege kyoki jab bhi user apni chal chlega toh 2d array me changes hoge. 
    let mut board = initialize_board();
    let mut current_player = PLAYER_X;

    loop {
        println!("Current Board:");
        // hum board 2d array ka reference pass krege.
        print_board(&board);

        // get_player_move function current player ki move ko lega aur usne jis position ki value di h ushe row, col me return kr dega.
        let (row, col) = get_player_move(current_player, &board);

        // board ki us row aur col vali position pr current_player ko place kr dege.
        board[row][col] = current_player;

        // ye function winner ko check krega, yadi koi user winner ban jata h toh loop ko break kr dege.
        if check_winner(current_player, &board){
            println!("Player {} is the winner", current_player);
            break;
        }

        // ye check krega ki match draw hua h ya nhi.
        if check_draw(&board){
            println!("The game is draw");
            break;
        }

        // iski help se hamara game dono player k bich me chlta rhega. jab PLAYER_X ki turn end ho jaye gi toh current_player me PLAYER_O  aa jaiye ga and vice versa.
        current_player = if current_player == PLAYER_X {
            PLAYER_O
        } else {
            PLAYER_X
        }
    }
}

fn main() {
    println!("Welcome to Tic-Tac-Toe Game!");

    play_game();
}
