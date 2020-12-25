import init from "./pkg/boa.js";
import * as wasm from "./pkg/boa.js";

const FIELD_WIDTH = 8;
const PLAYABLE_PLAYER = 1;

let NUMBER_PLAYOUTS = 2000;

let ai = null; 
let currstate = null; 
let wasmobj = null;

let bean_img = new Image();


initialize_wasm_object();

preload_images();

add_event_listener_to_buttons();

add_event_listener_to_slider();

// Check settigns for debug outptu
debug_output() 


async function initialize_wasm_object() {
    wasmobj = await init();

    wasm.init_panic_hook();   
    ai = new wasm.AI();
    currstate = new wasm.GameState();
}


function preload_images() {
    bean_img.onload = function(){ 
        console.log("Bean image preloaded.");
    }
    bean_img.src = "./images/bean_rot.png";
    var rotateAngle = 90;
    bean_img.setAttribute("style", "transform: rotate(" + rotateAngle + "deg)");
}

function add_event_listener_to_slider() {
    var slider = document.getElementById("playout_range");
    var information_text = document.getElementById("info_text_playout_number");
    slider.value = NUMBER_PLAYOUTS;
    information_text.innerHTML =  "Number playouts to think: " + NUMBER_PLAYOUTS;
    slider.oninput = function() {
        NUMBER_PLAYOUTS = this.value;
        information_text.innerHTML =  "Number playouts to think: " + NUMBER_PLAYOUTS;
    }
}

function add_event_listener_to_buttons() {    
    for (let player = 0; player < PLAYABLE_PLAYER; player++) {
        for (let position = 0; position < 2*FIELD_WIDTH; position++) {
            var elemid = '#p' + (player+1) + '_pos' + position;
            // console.log("Adding event listener for button " + elemid);
            document.querySelector(elemid).addEventListener('click', click_on_cell);
            document.querySelector(elemid).addEventListener('mouseover', mouse_over_cell);
            document.querySelector(elemid).addEventListener('mouseout', mouse_out_of_cell);

            elemid = '#ap' + (player+1) + '_pos' + position;
            // console.log("Adding event listener for button " + elemid);
            document.querySelector(elemid).addEventListener('click', click_on_cell);
            document.querySelector(elemid).addEventListener('mouseover', mouse_over_cell);
            document.querySelector(elemid).addEventListener('mouseout', mouse_out_of_cell);
        }
    }
    // document.querySelector('#nextmove').addEventListener('click', next_move);

    document.querySelector('#new_game_button').addEventListener('click', start_new_game);
    document.querySelector('#enable_debug_output').addEventListener('change', debug_output);
}

function start_new_game() {    
    currstate = new wasm.GameState();
    update_field_representation(currstate);
}

function debug_output() {
    var checkbox = document.querySelector('#enable_debug_output');
    var div_with_debug_info = document.querySelector('#debug_output_id');
    if (checkbox.checked == true) {
        div_with_debug_info.style.visibility = 'visible';
    } else {
        div_with_debug_info.style.visibility = 'hidden';
    }
}

function extract_button_position_from_element_id(elem_id) {
    if (elem_id.startsWith("p")) {
        return elem_id.substr(6);
    } else if (elem_id.startsWith("ap")) {
        return elem_id.substr(7);
    }
}

function get_color(num_stones) {
    if (num_stones == 0) {
        return "#cbcccd";
    } else if (num_stones > 0 && num_stones < 3) {
        return "#d5f5e3";
    } else if (num_stones >= 3 && num_stones < 5) {
        return "#e67e22";
    } else {
        return "#f51b1b";
    }    
}

function update_field_representation(gamestate) {
    const pre = document.getElementById("boa-viz");
    console.log("Updating game field");
    pre.textContent = gamestate.render();

    var c = document.getElementById("myCanvas");
    var img = document.getElementById("boardimg");
    c.style.position = "absolute";
    c.style.left = img.offsetLeft  + "px";
    c.style.top = img.offsetTop  + "px"; 
    
    var ctx = c.getContext("2d");
    ctx.font = "20px Georgia";
    ctx.clearRect(0, 0, c.width, c.height);

    for (let player = 0; player < 2; player++) {
        for (let position = 0; position < 2*FIELD_WIDTH; position++) {
            
                // Update information in the buttongs
                var elemid = '#p' + (player+1) + '_pos' + position;
                var elem = document.querySelector(elemid);
                var stones = gamestate.get_number_stones_at(player, position);
                //console.log("Received: " + " --> " + stones)
                elem.textContent = stones;
                elem.style.background=get_color(stones)


                // Update information in the image area
                
                var area_map_id = '#ap' + (player+1) + '_pos' + position;
                var areaelem = document.querySelector(area_map_id);
                var coords = areaelem.coords.split(",")
                
                // console.log("Drawing text on image at location: " + coords[0] + ", " + coords[1]);

                // Show number of beans on a field.
                ctx.fillText(stones, parseInt(coords[0]), parseInt(coords[1]));

                // Now, additionally draw that many beans as there are beans in field.
                // For that, load an image with a bean a put it at some place on the field.

                var img = bean_img;
                var coords0 = parseInt(coords[0]);
                var coords1 = parseInt(coords[1]);
                var num_rows = parseInt(stones / 2);
                var last_in_the_middle = (stones % 2 == 1);                
                var scale_factor = 0.5;
                var width_of_bean = 90 * scale_factor;
                var height_of_bean = 50 * scale_factor;
                var y_pixels_between_rows = height_of_bean;
                for (let index = 0; index < stones; index++) {
                    // console.log("Drawing bean at: " + (parseInt(coords0) + index*10) + ", " + (coords1+ index*10));
                    var curr_row = parseInt(index / 2);
                    var put_in_the_middle = last_in_the_middle && (index == stones - 1);
                    
                    if (put_in_the_middle) {
                        ctx.drawImage(img, coords0-width_of_bean/2, coords1 + (curr_row-(num_rows-1))*y_pixels_between_rows, width_of_bean, height_of_bean); 
                    } else {
                        if (index % 2 == 0) { // draw left
                            ctx.drawImage(img, coords0-(1.3*width_of_bean), coords1 + (curr_row-(num_rows-1))*y_pixels_between_rows, width_of_bean, height_of_bean); 
                        } else {
                            ctx.drawImage(img, coords0-width_of_bean/4, coords1 + (curr_row-(num_rows-1))*y_pixels_between_rows, width_of_bean, height_of_bean); 
                        }
                    }
                }                
        }        
    }
}

function click_on_cell(event) {
    // console.log("Currstate: " + currstate + ", ai: " + ai + "((currstate != null) && (ai != null)) " + ((currstate != null) && (ai != null)));
    if ((currstate != null) && (ai != null)) {
        var current_player = currstate.curr_player;
        // console.log("Clicked on button: event " + event);
        if (current_player == 0) {
            var position = extract_button_position_from_element_id(event.target.id);
            console.log("Clicked on pos=" + position);
            try {
                    let newstate = currstate.make_move_wasm(position);
                    update_field_representation(newstate);
                    if (newstate.game_over()) {
                        alert("Game over! Congratulations, you won!");
                    }
                    currstate = newstate;                
                    // console.log(newstate);
                    if (currstate != null && !currstate.game_over()) {                    
                        try {
                            // Now, AI makes a move
                            // console.log("Before doing AI step, the state is: " + currstate.render());                            
                            


                            
                            /////// NONE IF IT WORKS AS EXPECTED ///////////
                            // To show the progress, we do not calculate all steps at once but by small batches. This allows us to update the progress bar inbetween.
                            // And due to the current implemententation which does not do a real Monte Carlo search, it is ok like this.
                            var num_batches =  100; // 1 batch per 1% of progress
                            var playouts_per_batch = parseInt(NUMBER_PLAYOUTS / num_batches); 
                            if (playouts_per_batch == 0) { // user selected less than 100 playouts
                                num_batches = NUMBER_PLAYOUTS;
                                playouts_per_batch = 1;
                            }

                            const ai_progress = document.getElementById("ai_progress");
                            const chances_to_win_elem = document.getElementById("chances_to_win");
                            ai_progress.value = 0;
                            var diff_wins_distribution = null;

                            var loop = function (batch, played_playouts) {
                                // console.log("Batch: " + batch + " played_playouts: " + played_playouts + " playouts_per_batch: " + playouts_per_batch + " diff_wins_distribution: " + diff_wins_distribution, " num_batches: " + num_batches);
                                if (batch == num_batches - 1) {
                                    playouts_per_batch = NUMBER_PLAYOUTS - played_playouts;
                                }
                                let tmp = ai.evaluate_state_for_next_move(currstate, playouts_per_batch);
                                let tmp_diffs = new Int32Array(wasmobj.memory.buffer, tmp, 2*FIELD_WIDTH);
                                if (diff_wins_distribution == null) {
                                    diff_wins_distribution = tmp_diffs.slice();
                                } else {
                                    for (let index = 0; index < tmp_diffs.length; index++) {
                                        const element = tmp_diffs[index];
                                        diff_wins_distribution[index] += element;                                        
                                    }
                                }
                                
                                played_playouts += playouts_per_batch;                                
                                ai_progress.value = parseInt(played_playouts / NUMBER_PLAYOUTS * 100);
                                ai_progress.setAttribute('data-content', 'Played ' + ai_progress.value + ' playouts');

                                if (batch < num_batches) {
                                    setTimeout(function () {
                                        loop(batch + 1, played_playouts)
                                    }, 1); 
                                } else {
                                    console.log("After doing AI step, the winner diff is: " + diff_wins_distribution);
                                    let max_value = -NUMBER_PLAYOUTS-1;
                                    let max_index = 0;
                                    for (let index = 0; index < diff_wins_distribution.length; index++) {
                                        const value_at_index = diff_wins_distribution[index];
                                        if ((value_at_index > max_value) && (newstate.get_number_stones_at(1, index) > 0)) {
                                            max_value = value_at_index;
                                            max_index = index;
                                        }
                                    }
                                    console.log("Choose best position: " + max_index);
                                    let new_state_after_ai_move = newstate.make_move_wasm(max_index);                    
                                    update_field_representation(new_state_after_ai_move);
                                    if (new_state_after_ai_move.game_over()) {
                                        alert("Game over! Winner is Computer!");
                                    }
                                    currstate = new_state_after_ai_move;
                                    console.log("After doing AI step, the state is: " + new_state_after_ai_move.render());

                                    var win_chances_ai = 0;
                                    var win_chances_player = 0;
                                    for (let index = 0; index < diff_wins_distribution.length; index++) {
                                        const v = diff_wins_distribution[index];
                                        if (v < 0) {
                                            win_chances_player += diff_wins_distribution[index];
                                        } else {
                                            win_chances_ai += diff_wins_distribution[index];
                                        }

                                    }
                                    chances_to_win_elem.innerHTML = "Chances to win: (Computer) " + win_chances_ai + " vs " + (-win_chances_player) + " (Player)" 
                                }
                            }
                                                
                            loop(0, 0);

                            /*for (let batch = 0; batch < num_batches; batch += 1) {   
                                if (batch == num_batches - 1) {
                                    playouts_per_batch = NUMBER_PLAYOUTS - played_playouts;
                                }
                                let tmp = ai.evaluate_state_for_next_move(currstate, playouts_per_batch);
                                let tmp_diffs = new Int32Array(wasmobj.memory.buffer, tmp, 2*FIELD_WIDTH);
                                if (diff_wins_distribution == null) {
                                    diff_wins_distribution = tmp_diffs;
                                } else {
                                    for (let index = 0; index < tmp_diffs.length; index++) {
                                        const element = tmp_diffs[index];
                                        diff_wins_distribution[index] += element;                                        
                                    }
                                }
                                
                                played_playouts += playouts_per_batch;                                
                                ai_progress.value = parseInt(played_playouts / NUMBER_PLAYOUTS * 100);
                            }*/

                            /* --- BATCH VERSION --- */
                            /*
                            let tmp = ai.evaluate_state_for_next_move(currstate, NUMBER_PLAYOUTS);
                            let diff_wins_distribution = new Int32Array(wasmobj.memory.buffer, tmp, 2*FIELD_WIDTH);
                            
                            console.log("After doing AI step, the winner diff is: " + diff_wins_distribution);
                            let max_value = -NUMBER_PLAYOUTS-1;
                            let max_index = 0;
                            for (let index = 0; index < diff_wins_distribution.length; index++) {
                                const value_at_index = diff_wins_distribution[index];
                                if (value_at_index > max_value && newstate.get_number_stones_at(1, index)) {
                                    max_value = value_at_index;
                                    max_index = index;
                                }
                            }
                            console.log("Choose best position: " + max_index);
                            let new_state_after_ai_move = newstate.make_move_wasm(max_index);                    
                            update_field_representation(new_state_after_ai_move);
                            if (new_state_after_ai_move.game_over()) {
                                alert("Game over! Winner is Computer!");
                            }
                            currstate = new_state_after_ai_move;
                            console.log("After doing AI step, the state is: " + new_state_after_ai_move.render());
                            */
                        } catch (error) {
                            console.log("Error while making AI move: " + error);
                        }
                        
                    }

            } catch (error) {
                console.log("Error while making a move: " + error);
            }
        }        
    }
}

function mouse_over_cell(event) {
    if (currstate != null && currstate.curr_player === 0) {
        var position = extract_button_position_from_element_id(event.target.id);
        console.log("Mouse over pos=" + position);
        try {
            let newstate = currstate.make_move_wasm(position);
            update_field_representation(newstate);
        } catch (error) {
            console.log("Error while making a move: " + error);
        }
    }
}

function mouse_out_of_cell(event) {
    if (currstate != null && currstate.curr_player === 0) {
        update_field_representation(currstate);
    }
}

