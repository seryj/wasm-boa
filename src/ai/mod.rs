use crate::GameState;

struct StateEvaluation {
    stones_lost: u8,
    stones_won: u8,
    own_stones: u8,
    opponent_stones: u8,
}

// ///
// pub fn propose_move(start_state: &GameState, player: usize) -> usize {
//
// }
//
// fn simulate_game_of_depth_for_player_0(state: &GameState, last_own_move: usize, last_opponent_move: usize, depth: usize, currdepth: usize) -> StateEvaluation {
//     if currdepth == depth + 1 {
//         return  StateEvaluation {
//             stones_lost: 0,
//             stones_won: 0,
//             opponent_stones: 0,
//             own_stones: 0
//         };
//     } else if state.curr_player == 0 {
//         if last_own_move == crate::LENGTH_OF_FIELD - 1 {
//             return simulate_game_of_depth_for_player_0(&state.make_move(0).unwrap(), last_own_move+1, last_opponent_move, depth, currdepth+1);
//         } else {
//             return simulate_game_of_depth_for_player_0(&state.make_move(last_own_move+1).unwrap(), last_own_move+1, last_opponent_move, depth, currdepth);
//         }
//     } else { // if state.curr_player == 1 {
//         if last_opponent_move == crate::LENGTH_OF_FIELD {
//             return simulate_game_of_depth_for_player_0(&state.make_move(0).unwrap(), last_own_move+1, last_opponent_move, depth, currdepth+1);
//         } else {
//             return simulate_game_of_depth_for_player_0(&state.make_move(last_opponent_move+1).unwrap(), last_own_move, last_opponent_move+1, depth, currdepth));
//         }
//     }
// }
//
// fn asimulate_game_of_depth(state: &GameState, player_for_whom_propose_move: usize, depth: usize) -> StateEvaluation {
//     let virtual_player1_id: usize = if player_for_whom_propose_move == 0 {0} else {1};
//     let virtual_player2_id: usize = if player_for_whom_propose_move == 0 {1} else {0};
//
//     let mut path_of_moves_p1: Vec<GameState> = vec![];
//     let mut path_of_moves_p2: Vec<GameState> = vec![];
//     let mut lost_stones_p1: Vec<u8> = vec![];
//     let mut lost_stones_p1: Vec<u8> = vec![];
//
//     let mut curr_player = virtual_player1_id;
//     let mut curr_depth = 0;
//
//     let mut curr_best_
//
//     loop {
//         if curr_depth == depth {
//             break; // that's wrong
//         }
//
//         if let Some(currstsate) = path_of_moves.pop() {
//             match (currstsate.curr_player, player_for_whom_propose_move) {
//                 (0, 0) | (1, 1) => {
//                     let lastmove = own_last_positions.pop().unwrap();
//                     if lastmove >= crate::LENGTH_OF_FIELD {
//                         if player_for_whom_propose_move == 1 {
//                             if cu
//                             curr_depth += 1;
//
//                         }
//                     }
//                 },
//                 (1, 0) | (0, 1) => {
//                     let lastmove = opponent_last_positions.pop().unwrap();
//                 },
//                 _ => ()
//             }
//         }
//
//     }
//
//     StateEvaluation {
//         stones_lost: 0,
//         stones_won: 0,
//         opponent_stones: 0,
//         own_stones: 0
//     }
// }
//
// fn evaluate_state(state: GameState, player: usize) -> StateEvaluation {
//
// }
