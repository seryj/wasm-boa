use wasm_bindgen::__rt::std::time::Instant;

fn main() {
    let mut winner0 = 0;
    let mut winner1 = 0;
    let start = Instant::now();
    let mut unentschieden = 0;
    for _ in 0..5 {
        let game = boa::GameState::default(); // beginning of the game
        let winner_distr = boa::game_playout(game, None, Some(1000));

        match winner_distr {
            Ok(winner) => {
                println!(
                    "{:?}",
                    winner
                        .iter()
                        .map(|v| v.0 as i32 - v.1 as i32)
                        .collect::<Vec<i32>>()
                );
            }
            Err(boa::GameError::RepeatingPosition) => {}
            _ => panic!("ohoh..."),
        }

        // if start.elapsed().as_millis() > 2000 {
        //     break;
        // }
    }

    println!("W1: {}, W2: {}, both: {}", winner0, winner1, unentschieden);
}
