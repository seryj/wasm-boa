import * as wasm from "boa";
import { memory } from "boa/boa_bg";

const FIELD_WIDTH = 8;
const PLAYABLE_PLAYER = 1;
const startstate = new wasm.GameState();

let NUMBER_PLAYOUTS = 2000;

let currstate = startstate;

let bean_img = new Image();

preload_images();

add_event_listener_to_buttons();

add_event_listener_to_slider();

wasm.init_panic_hook();

function preload_images() {
    bean_img.onload = function(){ 
        console.log("Bean image preloaded.");
    }
    bean_img.src = "assets/bean.png"
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
                var y_pixels_between_rows = 30;
                for (let index = 0; index < stones; index++) {
                    // console.log("Drawing bean at: " + (parseInt(coords0) + index*10) + ", " + (coords1+ index*10));
                    var curr_row = parseInt(index / 2);
                    var put_in_the_middle = last_in_the_middle && (index == stones - 1);
                    
                    if (put_in_the_middle) {
                        ctx.drawImage(img, coords0, coords1 + (curr_row-(num_rows-1))*y_pixels_between_rows, 40, 30); 
                    } else {
                        if (index % 2 == 0) { // draw left
                            ctx.drawImage(img, coords0-50, coords1 + (curr_row-(num_rows-1))*y_pixels_between_rows, 40, 30); 
                        } else {
                            ctx.drawImage(img, coords0, coords1 + (curr_row-(num_rows-1))*y_pixels_between_rows, 40, 30); 
                        }
                    }
                }                
        }        
    }
}

function click_on_cell(event) {
    if (currstate != null) {
        var current_player = currstate.curr_player;
        // console.log("Clicked on button: event " + event);
        if (current_player == 0) {
            var position = extract_button_position_from_element_id(event.target.id);
            //console.log("Clicked on pos=" + position);
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
                        console.log("Before doing AI step, the state is: " + currstate.render());
                        let diff_wins_distribution = new Int32Array(memory.buffer, wasm.game_playout_wasm(currstate, NUMBER_PLAYOUTS), 2*FIELD_WIDTH);
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

