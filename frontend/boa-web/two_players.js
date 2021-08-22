import * as wasm from "./pkg/boa.js";

const FIELD_WIDTH = 8;     // number of fields in a single row
let player_unique_id = null;
let curr_game_state = null;

let bean_img = new Image(); // preloaded bean image 

websocket = null;


preload_images();

add_event_listener_to_buttons();

// Check settigns for debug output
debug_output() 


function connect_to_server(url) {
    url = "wss://localhots:9090";
    websocket = new WebSocket(url, protocols);

    websocket.onopen = function(event) {
        // On initial connection, the server sends will notify us with a message with a random ID which we will 
        // use for identification
        websocket.send(JSON.stringify(
            {
                "event": "wanna play"
            }
        ));
    }

    websocket.onmessage = function (event) {
        console.log(event.data);
        var server_msg = JSON.parse(event.data);

        if (server_msg['msg_type'] == "INIT_CONN") {
            player_unique_id = parseInt(server_msg["unique_id"]);
        } else if (server_msg['msg_type'] == "PLAYOUT") {
            other_players_move = parseInt(server_msg["move"]);
            curr_game_state = server_msg["gamestate"]
        }
    }
}

function send_move(position) {
    if ((websocket != null) && (player_unique_id != null)) {
        var msg = {
            player_id: player_unique_id,
            move: position
        };
        websocket.send(JSON.stringify(msg));
    }
}


/**
 * Don't remember where I got the got from but it should preload bean images such that they are not loaded every time they need to be drawn.
 */
function preload_images() {
    bean_img.onload = function(){ 
        console.log("Bean image preloaded.");
    }
    bean_img.src = "./images/bean_rot.png";
    var rotateAngle = 90;
    bean_img.setAttribute("style", "transform: rotate(" + rotateAngle + "deg)");
}


/**
 * Adds click, mouseover and mouseout event listeneres to the buttons in the debug output and to the map shapes. Moreover, to all
 * other buttons, checkboxes, etc..
 */
function add_event_listener_to_buttons() {    

    for (let position = 0; position < 2*FIELD_WIDTH; position++) {
        var elemid = '#p1_pos' + position;
        // console.log("Adding event listener for button " + elemid);
        document.querySelector(elemid).addEventListener('click', click_on_cell);
        document.querySelector(elemid).addEventListener('mouseover', mouse_over_cell);
        document.querySelector(elemid).addEventListener('mouseout', mouse_out_of_cell);

        elemid = '#ap1_pos' + position;
        // console.log("Adding event listener for button " + elemid);
        document.querySelector(elemid).addEventListener('click', click_on_cell);
        document.querySelector(elemid).addEventListener('mouseover', mouse_over_cell);
        document.querySelector(elemid).addEventListener('mouseout', mouse_out_of_cell);
    }

    // document.querySelector('#nextmove').addEventListener('click', next_move);

    document.querySelector('#new_game_button').addEventListener('click', start_new_game);
    document.querySelector('#enable_debug_output').addEventListener('change', debug_output);
    document.querySelector('#connect_to_server_btn').addEventListener('click', connect_to_server);
}

/**
 * Start new game.
 */
function start_new_game() {    
    update_field_representation(curr_game_state);
}

/**
 * Enables or disables debug output.
 */
function debug_output() {
    var checkbox = document.querySelector('#enable_debug_output');
    var div_with_debug_info = document.querySelector('#debug_output_id');
    if (checkbox.checked == true) {
        div_with_debug_info.style.visibility = 'visible';
    } else {
        div_with_debug_info.style.visibility = 'hidden';
    }
}

/**
 * The buttons and the map shapes are named in a way that one can extract the field position from the name
 * of the element id. Exactly this happens here: given the DOM element id, it return the location of the field in the corresponding internal array.
 * 
 * @param {*} elem_id 
 */
function extract_button_position_from_element_id(elem_id) {
    if (elem_id.startsWith("p")) {
        return elem_id.substr(6);
    } else if (elem_id.startsWith("ap")) {
        return elem_id.substr(7);
    }
}

/**
 * A mapping for number of beans in a field --> color of the button in the debug output.
 * 
 * @param {*} num_stones Number of beans in in a field.
 */
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

/**
 * Update the UI (move the beans, etc.).
 * 
 * @param {*} gamestate Gamestate that should be shown on the UI.
 */
function update_field_representation(gamestate) {
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
            
                // Update information in the buttons
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

/**
 * Handle what happens when an area map or a button in the debug output is clicked (basically, to the move...). In more details, two things happen.
 * First, if it's player turn, clicking on a field will make a move for the player. Afterwards, the evaluation for the best move of AI is triggered.
 * This blocks the main thread, but... well, it's like it is. After the AI provides an evaluaiton for all positions, the best move is selected and done. 
 * @param {*} event Click event.
 */
function click_on_cell(event) {
    // console.log("Currstate: " + currstate + ", ai: " + ai + "((currstate != null) && (ai != null)) " + ((currstate != null) && (ai != null)));
    if ((currstate != null) && (ai != null)) {
        var current_player = currstate.curr_player;
        // console.log("Clicked on button: event " + event);
        if (current_player == 0) {            
            var position = extract_button_position_from_element_id(event.target.id);
            console.log("Clicked on pos=" + position);
            try {
                // -------------------
                // Here, we make the move of the player.
                websocket.send(position);
                update_field_representation(newstate);
                if (newstate.game_over()) {
                    alert("Game over! Congratulations, you won!");
                }
                currstate = newstate;                
                // console.log(newstate);

                // ---------------------
                // Here, the block comes, where AI evaluates a move and the actual move is done.
                if (currstate != null && !currstate.game_over()) {                    
                    try {
                        // now, we wait for other player to make a move
                        send_move(position);
                    } catch (error) {
                        console.log("Error while sending the move to the server: " + error);
                    }
                    
                }

            } catch (error) {
                console.log("Error while making a move: " + error);
            }
        }        
    }
}

/**
 * On mouse-over over own part of the game field, we show the distribution of beans if one does a move from this location.
 * 
 * @param {*} event Mouse event.
 */
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

/**
 * If the mouse leaves the area map, we show the original state of the game.
 * @param {*} event 
 */
function mouse_out_of_cell(event) {
    if (currstate != null && currstate.curr_player === 0) {
        update_field_representation(currstate);
    }
}

