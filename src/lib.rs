#![feature(test)]

extern crate console_error_panic_hook;
// mod ai;

use crate::Direction::Left;
use rand::Rng;
use std::fmt::{Display, Formatter};
use std::time::{Duration, Instant};
use wasm_bindgen::__rt::core::cmp::max;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

const LENGTH_OF_FIELD: usize = 8;

#[repr(u8)]
#[derive(Debug)]
pub enum GameError {
    PositionOutsideField,
    CellEmpty,
    NoMoveFound,
    RepeatingPosition,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct MoveStatistic {
    pub last_move_position: usize,
    pub stones_of_opponent_removed: u8,
}

impl Default for MoveStatistic {
    fn default() -> Self {
        MoveStatistic {
            last_move_position: 0,
            stones_of_opponent_removed: 0,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct OnePlayersField {
    /// direction of movement
    direction: Direction,
    /// Number of stones in the own half of the field.
    cells: [u8; 2 * LENGTH_OF_FIELD],
}

/// State of the game
#[wasm_bindgen]
#[derive(Clone)]
pub struct GameState {
    /// The half's of the boards belonging to each player
    // pub fields_of_players: Vec<OnePlayersField>,
    fields_of_players: [OnePlayersField; 2],
    /// Players: 0 or 1
    pub curr_player: u8,
    /// Game over?
    pub game_over: bool,
    /// Statistics after last move
    pub move_statistic: MoveStatistic,
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
impl OnePlayersField {
    fn new(direction: Direction) -> Self {
        OnePlayersField {
            direction: direction,
            cells: [2; 2 * LENGTH_OF_FIELD],
        }
    }

    /// Return new distribution of stones on the field and the position of the stone which was
    /// put last.
    fn make_move_from(&self, position: usize) -> Result<(Self, usize), GameError> {
        if position > self.cells.len() {
            return Err(GameError::PositionOutsideField);
        }

        if self.cells[position] <= 0 {
            return Err(GameError::CellEmpty);
        }

        let mut new_field_state = self.cells.clone();
        let mut curr_pos = position;
        let mut stones_in_the_hand = self.cells[curr_pos];
        new_field_state[curr_pos] = 0;
        loop {
            // println!("{} pos({})-- {:?}", stones_in_the_hand, curr_pos, new_field_state);

            curr_pos = if self.direction == Direction::Right {
                (curr_pos + 1) % self.cells.len()
            } else {
                (curr_pos + (2 * LENGTH_OF_FIELD - 1)) % self.cells.len() // corresponds to make a step "backward"
            };

            new_field_state[curr_pos] += 1;
            stones_in_the_hand -= 1;
            if (stones_in_the_hand == 0) && (new_field_state[curr_pos] == 1) {
                break;
            } else if (stones_in_the_hand == 0) && (new_field_state[curr_pos] > 1) {
                if self
                    .cells
                    .iter()
                    .zip(new_field_state.iter())
                    .all(|(v1, v2)| *v1 == *v2)
                {
                    return Err(GameError::RepeatingPosition);
                }

                stones_in_the_hand = new_field_state[curr_pos];
                new_field_state[curr_pos] = 0;
            }
        }

        return Ok((
            OnePlayersField {
                direction: self.direction,
                cells: new_field_state,
            },
            curr_pos,
        ));
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            fields_of_players: [
                OnePlayersField::new(Direction::Left),
                OnePlayersField::new(Direction::Right),
            ],
            curr_player: 0,
            game_over: false,
            move_statistic: MoveStatistic::default(),
        }
    }
}

#[wasm_bindgen]
impl GameState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GameState {
        GameState::default()
    }

    fn game_over_internal(&self, fields: &[OnePlayersField]) -> bool {
        (fields[0].cells.iter().sum::<u8>() == 0) || (fields[1].cells.iter().sum::<u8>() == 0)
    }

    pub fn game_over(&self) -> bool {
        self.winner().is_some()
    }

    fn winner(&self) -> Option<u8> {
        if self.fields_of_players[0].cells.iter().sum::<u8>() == 0 {
            Some(1)
        } else if self.fields_of_players[1].cells.iter().sum::<u8>() == 0 {
            Some(0)
        } else {
            None
        }
    }

    fn get_field_copy(&self) -> [OnePlayersField; 2] {
        [
            self.fields_of_players[0].clone(),
            self.fields_of_players[1].clone(),
        ]
    }

    /// Makes a move for the current user and returns a copy of the new GameState
    fn make_move(&self, position: usize) -> Result<GameState, GameError> {
        let other_player: usize = (self.curr_player as usize + 1) % 2;
        let curr_player: usize = self.curr_player as usize;

        let new_field_state_of_current_player =
            self.fields_of_players[curr_player].make_move_from(position);

        match new_field_state_of_current_player {
            Err(e) => return Err(e),
            Ok(new_field_and_last_location_curr_user) => {
                // Now, remove stones from the other player if needed
                let mut other_player_field = self.fields_of_players[other_player].clone();
                let mut removed_stones_of_opponent: u8 = 0;

                if new_field_and_last_location_curr_user.1 < LENGTH_OF_FIELD {
                    if self.fields_of_players[other_player].cells
                        [new_field_and_last_location_curr_user.1]
                        > 0
                    {
                        // remove stones
                        removed_stones_of_opponent +=
                            other_player_field.cells[new_field_and_last_location_curr_user.1];
                        removed_stones_of_opponent += other_player_field.cells
                            [2 * LENGTH_OF_FIELD - new_field_and_last_location_curr_user.1 - 1];

                        other_player_field.cells[new_field_and_last_location_curr_user.1] = 0;
                        other_player_field.cells
                            [2 * LENGTH_OF_FIELD - new_field_and_last_location_curr_user.1 - 1] = 0;
                    }
                }

                let fields_of_both_players = if curr_player == 0 {
                    [
                        new_field_and_last_location_curr_user.0.clone(),
                        other_player_field,
                    ]
                } else {
                    [
                        other_player_field,
                        new_field_and_last_location_curr_user.0.clone(),
                    ]
                };

                let move_statistic = MoveStatistic {
                    stones_of_opponent_removed: removed_stones_of_opponent,
                    last_move_position: new_field_and_last_location_curr_user.1,
                };

                let game_over = self.game_over_internal(&fields_of_both_players);
                Ok(GameState {
                    curr_player: other_player as u8,
                    fields_of_players: fields_of_both_players,
                    game_over: game_over,
                    move_statistic: move_statistic,
                })
            }
        }
    }

    pub fn make_move_wasm(&self, position: usize) -> Result<GameState, JsValue> {
        if position >= 2 * LENGTH_OF_FIELD {
            return Err(JsValue::from("Position outside of field"));
        }
        let new_state = self.make_move(position);
        match new_state {
            Ok(state) => Ok(state),
            Err(e) => Err(JsValue::from(
                format!("Error while doing a move: {:?}", e).as_str(),
            )),
        }
    }

    fn get_position_from_row_col(&self, player: usize, row: usize, col: usize) -> usize {
        let position = if player == 0 {
            row * LENGTH_OF_FIELD + (LENGTH_OF_FIELD - 1 - col)
        } else {
            row * LENGTH_OF_FIELD + (col)
        };

        position
    }

    fn make_move_via_row_col(&self, row: usize, col: usize) -> Result<GameState, GameError> {
        if col >= LENGTH_OF_FIELD {
            return Err(GameError::PositionOutsideField);
        }

        let position = self.get_position_from_row_col(self.curr_player as usize, row, col);
        self.make_move(position)
    }

    /// Makes move using row and column notation.
    /// Row and column are always relative to the user. I.e, it looks from his/her position on the board and the bottom row is the 0th, and top row is top 1st one.
    /// The columns are counted from left to right.
    pub fn make_move_via_row_col_wasm(&self, row: usize, col: usize) -> Result<GameState, JsValue> {
        let new_state = self.make_move_via_row_col(row, col);
        match new_state {
            Ok(state) => Ok(state),
            Err(e) => Err(JsValue::from(
                format!("Error while doing a move: {:?}", e).as_str(),
            )),
        }
    }

    pub fn get_number_stones_at(&self, player: usize, position: usize) -> Result<u8, JsValue> {
        assert!(position < 2 * LENGTH_OF_FIELD);
        assert!(player < 2);
        Ok(self.fields_of_players[player].cells[position])
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

fn get_random_number(from: usize, to: usize) -> usize {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut rng = rand::thread_rng();
        return rng.gen_range(from, to);
    };

    #[cfg(target_arch = "wasm32")]
    {
        return (js_sys::Math::random() * (to as f64)) as usize + from;
    }
}

pub fn random_playout(state: GameState) -> Result<u8, GameError> {
    let mut newstate: GameState = state;
    let mut abort_counter = 100000;
    let mut list_of_moves_for_debug: Vec<GameState> = vec![];

    loop {
        assert!(
            abort_counter > 0,
            format!(
                "Could not find a move: move history: {}",
                list_of_moves_for_debug
                    .iter()
                    .map(|state| format!("{}", state))
                    .collect::<Vec<String>>()
                    .join("\n")
            )
        );
        abort_counter -= 1;

        if let Some(winner) = newstate.winner() {
            return Ok(winner);
        }

        // if newstate.fields_of_players[0].cells.iter().sum::<u8>() == 1
        //     && newstate.fields_of_players[0].cells.iter().sum::<u8>() == 1
        //     && abort_counter < 100
        // {
        //     return Err(GameError::NoMoveFound);
        // }

        let field_from_which_to_make_next_step =
            &newstate.fields_of_players[newstate.curr_player as usize];
        let non_empty_cell_ids: Vec<usize> = field_from_which_to_make_next_step
            .cells
            .iter()
            .enumerate()
            .filter(|(_, stones_in_cell)| **stones_in_cell > 0)
            .map(|(cell_id, _)| cell_id)
            .collect();

        assert!(non_empty_cell_ids.len() > 0);
        // println!("{:?}", non_empty_cell_ids);

        let random_move_id = if non_empty_cell_ids.len() > 1 {
            get_random_number(0, non_empty_cell_ids.len())
        } else {
            0
        };
        let newstate_result = newstate.make_move(non_empty_cell_ids[random_move_id]);
        if newstate_result.is_ok() {
            newstate = newstate_result.unwrap();
        //list_of_moves_for_debug.push(newstate.clone());
        } else {
            let error = newstate_result.err().unwrap();
            // println!(
            //     "Could not find a move: error {:?}, position: {} (from: {:?}), field: {}",
            //     error, random_move_id, non_empty_cell_ids, newstate
            // );
            return Err(error);
        }
    }
}

/// Make a complete random game playout. The result is the distribution of wins/losts for every
/// position on the field.
pub fn game_playout(
    state: GameState,
    num_rounds: Option<u32>,
    max_time_in_millis: Option<u128>,
) -> Result<[(u32, u32); 2 * LENGTH_OF_FIELD], GameError> {
    let mut wins_losses: [(u32, u32); 2 * LENGTH_OF_FIELD] = [(0, 0); 2 * LENGTH_OF_FIELD];
    let mut rounds_counter = 0;

    #[cfg(not(target_arch = "wasm32"))]
    let starttime = { Instant::now() };

    loop {
        for pos_on_board in 0..(2 * LENGTH_OF_FIELD) {
            let newstate = state.make_move(pos_on_board);
            if let Ok(newstate) = newstate {
                if let Some(winnerid) = newstate.winner() {
                    wins_losses[pos_on_board] = if winnerid == 0 {
                        (wins_losses[pos_on_board].0 + 1, wins_losses[pos_on_board].1)
                    } else {
                        (wins_losses[pos_on_board].0, wins_losses[pos_on_board].1 + 1)
                    };
                } else {
                    let winner = random_playout(newstate.clone());
                    if let Ok(winnerid) = winner {
                        wins_losses[pos_on_board] = if winnerid == 0 {
                            (wins_losses[pos_on_board].0 + 1, wins_losses[pos_on_board].1)
                        } else {
                            (wins_losses[pos_on_board].0, wins_losses[pos_on_board].1 + 1)
                        };
                    }
                }
            }
        }

        rounds_counter += 1;

        // ----- START: Stop conditions --------
        if let Some(num_rounds) = num_rounds {
            if num_rounds < rounds_counter {
                return Ok(wins_losses);
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(max_time_in_millis) = max_time_in_millis {
                if starttime.elapsed().as_millis() > max_time_in_millis {
                    return Ok(wins_losses);
                }
            }
        }
        // ----- END: Stop conditions --------
    }
}

/// Check for best move.
/// :num_rounds: If > 0 then calculate max num_rounds playouts for every posisition.
/// :max_time_to_think_in_millis: If num_rounds < 0 and max_time_to_think_in_millis > 0 then do playout until the maximal time to think is not over.
/// If both parameters are < 0, the default is to play 100 games.
#[wasm_bindgen]
pub fn game_playout_wasm(
    state: &GameState,
    num_rounds: i32,
    max_time_to_think_in_millis: u32,
) -> Result<*const i32, JsValue> {
    let curr_player = state.curr_player;
    let result = if num_rounds > 0 || max_time_to_think_in_millis <= 0 {
        if num_rounds < 0 {
            game_playout(state.clone(), Some(100), None) // default
        } else {
            game_playout(state.clone(), Some(num_rounds as u32), None)
        }
    } else {
        game_playout(
            state.clone(),
            None,
            Some(max_time_to_think_in_millis as u128),
        )
    };
    match result {
        Ok(wins_losses) => {
            let wins_of_curr_player = wins_losses
                .iter()
                .map(|v| {
                    if curr_player == 0 {
                        v.0 as i32 - v.1 as i32
                    } else {
                        v.1 as i32 - v.0 as i32
                    }
                })
                .collect::<Vec<i32>>();

            Ok(wins_of_curr_player.as_ptr())
        }
        Err(e) => Err(JsValue::from(format!("Error: {:?}", e))),
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match f.write_str(format!("Current player: {}, stones removed in last move: {}, game over: {}, player 1 stones: {}, player 2 stones: {}\n{:?}\n{:?} Player 2\n------------------------------\n{:?}  Player 1\n{:?}",
                            self.curr_player,  self.move_statistic.stones_of_opponent_removed, self.game_over, // some statistics
                            self.fields_of_players[0].cells.iter().sum::<u8>(), self.fields_of_players[1].cells.iter().sum::<u8>(),

                            self.fields_of_players[1].cells[LENGTH_OF_FIELD..(2*LENGTH_OF_FIELD)].iter().rev().collect::<Vec<&u8>>(),
                            self.fields_of_players[1].cells[0..LENGTH_OF_FIELD].to_vec(),
                            self.fields_of_players[0].cells[0..LENGTH_OF_FIELD].to_vec(),
                            self.fields_of_players[0].cells[LENGTH_OF_FIELD..(2*LENGTH_OF_FIELD)].iter().rev().collect::<Vec<&u8>>(),
        ).as_str()
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    #![feature(test)]
    extern crate test;
    use wasm_bindgen_test::*;

    use crate::{
        game_playout, game_playout_wasm, get_random_number, Direction, GameError, GameState,
        MoveStatistic, OnePlayersField,
    };
    use rand::Rng;
    use std::rc::Rc;
    use test::Bencher;
    use wasm_bindgen::__rt::std::time::Instant;

    #[wasm_bindgen_test]
    fn wasm_test_playout() {
        let state = GameState::default();
        let res = game_playout_wasm(&state, 1i32, 0);

        assert!(res.is_ok());
    }

    #[test]
    fn move_statistic_is_correct() {
        let gamestate1 = GameState {
            game_over: false,
            move_statistic: MoveStatistic::default(),
            curr_player: 0,
            fields_of_players: [
                OnePlayersField {
                    // Player 1, moves to the right
                    direction: Direction::Right,
                    cells: [
                        0, 0, 0, 1, 0, 0, 0, 0, // 0   1  2  3  4  5 6 7
                        0, 0, 0, 0, 0, 0, 0,
                        0, // 15 14 13 12 11 10 9 8 // corresponds to "0,0,0,0,1,0,0,0" when reversed
                    ],
                },
                OnePlayersField {
                    // Player 2, moves to the left
                    direction: Direction::Left,
                    cells: [
                        1, 0, 0, 0, 1, 0, 0, 0, // 15 14 13 12 11 10 9 8
                        0, 0, 0, 0, 0, 0, 0,
                        0, // 0   1  2  3  4  5 6 7   // corresponds to "0,0,0,1,0,0,0,0" when reversed
                    ],
                },
            ],
        };

        let new_state = gamestate1.make_move(3);

        assert!(new_state.is_ok(), "Could not make a move :(");
        let new_state = new_state.ok().unwrap();
        assert_eq!(new_state.move_statistic.last_move_position, 4);
        assert_eq!(new_state.move_statistic.stones_of_opponent_removed, 1);

        let state_after_second_move = new_state.make_move(0);
        assert!(state_after_second_move.is_ok(), "Could not make a move :(");
        let state_after_second_move = state_after_second_move.ok().unwrap();
        assert_eq!(
            state_after_second_move.move_statistic.last_move_position,
            15
        );
        assert_eq!(
            state_after_second_move
                .move_statistic
                .stones_of_opponent_removed,
            0
        );
    }

    #[test]
    fn game_over_works() {
        let gamestate1 = GameState {
            game_over: false,
            move_statistic: MoveStatistic::default(),
            curr_player: 0,
            fields_of_players: [
                OnePlayersField {
                    // Player 1, moves to the right
                    direction: Direction::Right,
                    cells: [
                        0, 0, 0, 1, 0, 0, 0, 0, // 0   1  2  3  4  5 6 7
                        0, 0, 0, 1, 0, 0, 0, 0,
                    ], // 15 14 13 12 11 10 9 8 // corresponds to "0,0,0,0,1,0,0,0" when reversed
                },
                OnePlayersField {
                    // Player 2, moves to the left
                    direction: Direction::Left,
                    cells: [
                        0, 0, 0, 0, 0, 0, 0, 0, // 15 14 13 12 11 10 9 8
                        0, 0, 0, 0, 0, 0, 0, 0,
                    ], // 0   1  2  3  4  5 6 7   // corresponds to "0,0,0,1,0,0,0,0" when reversed
                },
            ],
        };

        let gamestate2 = GameState {
            game_over: false,
            move_statistic: MoveStatistic::default(),
            curr_player: 0,
            fields_of_players: [
                OnePlayersField {
                    // Player 1, moves to the right
                    direction: Direction::Right,
                    cells: [
                        0, 0, 0, 0, 0, 0, 0, 0, // 0   1  2  3  4  5 6 7
                        0, 0, 0, 0, 0, 0, 0, 0,
                    ], // 15 14 13 12 11 10 9 8
                },
                OnePlayersField {
                    // Player 2, moves to the left
                    direction: Direction::Left,
                    cells: [
                        0, 0, 0, 1, 0, 0, 0, 0, // 15 14 13 12 11 10 9 8
                        0, 0, 0, 0, 0, 0, 0, 0,
                    ], // 0   1  2  3  4  5 6 7
                },
            ],
        };

        let gamestate3 = GameState {
            game_over: false,
            move_statistic: MoveStatistic::default(),
            curr_player: 0,
            fields_of_players: [
                OnePlayersField {
                    // Player 1, moves to the right
                    direction: Direction::Right,
                    cells: [
                        0, 1, 0, 0, 0, 0, 0, 0, // 0   1  2  3  4  5 6 7
                        0, 0, 0, 0, 0, 0, 0, 0,
                    ], // 15 14 13 12 11 10 9 8
                },
                OnePlayersField {
                    // Player 2, moves to the left
                    direction: Direction::Left,
                    cells: [
                        0, 0, 0, 1, 0, 0, 0, 0, // 15 14 13 12 11 10 9 8
                        0, 0, 0, 0, 0, 0, 0, 0,
                    ], // 0   1  2  3  4  5 6 7
                },
            ],
        };

        assert_eq!(gamestate1.game_over(), true);
        assert_eq!(gamestate2.game_over(), true);
        assert_eq!(gamestate3.game_over(), false);
    }

    #[test]
    fn test_move_deletes_stone_of_opposite() {
        let gamestate = GameState {
            game_over: false,
            move_statistic: MoveStatistic::default(),
            curr_player: 0,
            fields_of_players: [
                OnePlayersField {
                    // Player 1, moves to the right
                    direction: Direction::Right,
                    cells: [
                        0, 0, 0, 1, 0, 0, 0, 0, // 0   1  2  3  4  5 6 7
                        0, 0, 0, 1, 0, 0, 0, 0,
                    ], // 15 14 13 12 11 10 9 8 // corresponds to "0,0,0,0,1,0,0,0" when reversed
                },
                OnePlayersField {
                    // Player 2, moves to the left
                    direction: Direction::Left,
                    cells: [
                        0, 0, 0, 0, 1, 0, 0, 0, // 15 14 13 12 11 10 9 8
                        0, 0, 0, 0, 1, 0, 0, 0,
                    ], // 0   1  2  3  4  5 6 7   // corresponds to "0,0,0,1,0,0,0,0" when reversed
                },
            ],
        };
        let new_state = gamestate.make_move(3);
        assert_eq!(
            new_state.is_ok(),
            true,
            "{}",
            format!("{:?}", new_state.err().unwrap()).as_str()
        );
        let new_state = new_state.ok().unwrap();
        assert_eq!(new_state.fields_of_players[0].cells[4], 1);
        assert_eq!(
            new_state.fields_of_players[1].cells.iter().sum::<u8>(),
            1,
            "{}",
            format!("Old state: {}\n New state: {}", gamestate, new_state).as_str()
        );
    }

    #[test]
    fn test_game_stops_when_game_over() {
        let mut game = GameState::default(); // beginning of the game
        let mut new_game_state: GameState;
        let mut counter: usize = 0;
        let mut abort_counter = 1000;
        for _ in 0..1000 {
            loop {
                while game.make_move(counter).is_err() && !game.game_over() && abort_counter > 0 {
                    counter = game.fields_of_players[game.curr_player as usize]
                        .cells
                        .iter()
                        .enumerate()
                        .filter(|v| *v.1 > 0u8)
                        .map(|v| v.0)
                        .collect::<Vec<usize>>()[0];
                    abort_counter -= 1;
                }
                if game.game_over || abort_counter == 0 {
                    // println!("Game over");
                    break;
                }

                game = game.make_move(counter).ok().unwrap();
                // println!("{}", game);

                abort_counter -= 1;
            }
            assert!(abort_counter > 0);
            assert!(game.game_over(), "Game should be over.");
        }
    }

    #[test]
    fn test_game_playout_distribution() {
        let gamestate = GameState {
            game_over: false,
            move_statistic: MoveStatistic::default(),
            curr_player: 0,
            fields_of_players: [
                OnePlayersField {
                    // Player 1, moves to the right
                    direction: Direction::Right,
                    cells: [
                        0, 0, 0, 1, 0, 0, 0, 0, // 0   1  2  3  4  5 6 7
                        0, 0, 0, 0, 0, 0, 0, 0, // 15 14 13 12 11 10 9 8
                    ],
                },
                OnePlayersField {
                    // Player 2, moves to the left
                    direction: Direction::Left,
                    cells: [
                        0, 0, 0, 0, 1, 0, 0, 0, // 15 14 13 12 11 10 9 8
                        0, 0, 0, 0, 0, 0, 0, 0, // 0   1  2  3  4  5 6 7
                    ],
                },
            ],
        };

        let res = game_playout(gamestate, Some(1), None);
        assert!(res.is_ok());
        let distr = res.unwrap();
        assert!(distr[3].0 > 0, "{:?}", distr);
    }

    #[test]
    fn test_full_playout() {
        let game = GameState::default(); // beginning of the game
        let winner = game_playout(game, Some(1), None);
        assert!(winner.is_ok());
    }

    #[test]
    fn test_full_playout_100_times() {
        for _ in 0..100 {
            let game = GameState::default(); // beginning of the game
            let winner = game_playout(game, Some(1), None);
            assert!(winner.is_ok());
        }
    }

    #[bench]
    fn bench_one_move(b: &mut Bencher) {
        b.iter(|| {
            let game = GameState::default(); // beginning of the game
            game.make_move(0);
        });
    }

    #[bench]
    fn bench_playout_from_full_field(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(1);
            for _ in 0..n {
                let game = GameState::default(); // beginning of the game
                let winner = game_playout(game, Some(1), None);
            }
        });
    }

    #[bench]
    fn create_random_number(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(1);
            for _ in 0..n {
                get_random_number(0, 10);
            }
        });
    }
}
