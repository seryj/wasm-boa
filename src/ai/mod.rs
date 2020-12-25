use crate::{GameState, GameError, LENGTH_OF_FIELD};
use wasm_bindgen::JsValue;
use std::time::Instant;
use rand::Rng;

use wasm_bindgen::prelude::*;

pub fn get_random_number(from: usize, to: usize) -> usize {
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

#[wasm_bindgen]
pub struct AI {

}

#[wasm_bindgen]
impl AI {
    #[wasm_bindgen(constructor)]
    pub fn new() -> AI {
        return AI {};
    }

    /// Check for best move.
    /// :num_rounds: If > 0 then calculate max num_rounds playouts for every position.
    /// :max_time_to_think_in_millis: If num_rounds < 0 and max_time_to_think_in_millis > 0 then do playout until the maximal time to think is not over.
    /// If both parameters are < 0, the default is to play 100 games.
    pub fn evaluate_state_for_next_move(
        &self,
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
}

/// Plays a random game from the current game state. I.e., choose as long random moves by player
/// 1 and 2 until the game is over. When the game is over, return the winner ID (0 or 1). If
/// the game could not be finished (in some very seldom game states, there is no way to end the
/// game because no move removes any additional beans.
pub fn random_playout(state: GameState) -> Result<u8, GameError> {
    let mut newstate: GameState = state;
    let mut abort_counter = 100000;
    loop {
        assert!(
            abort_counter > 0,
            format!("Could not complete the playout.")
        );
        abort_counter -= 1;

        if let Some(winner) = newstate.winner() {
            return Ok(winner);
        }

        let field_from_which_to_make_next_step =
            &newstate.fields_of_players[newstate.curr_player as usize];

        // Get only non-empty cells from which one can make a move.
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
        } else {
            let error = newstate_result.err().unwrap();
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

    #[cfg(not(target_arch = "wasm32"))] let starttime = { Instant::now() };

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

