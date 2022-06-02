use junglechess::ai::get_ai_move;
use junglechess::board::{Board, Ground, Piece, Player};
use std::ptr;
use wasm_bindgen::prelude::*;

static mut BOARD_PTR: *mut Board = ptr::null_mut();

#[wasm_bindgen(module = "/index.js")]
extern "C" {
    pub fn clear_board();
    pub fn put_grass(x: isize, y: isize);
    pub fn put_water(x: isize, y: isize);
    pub fn put_trap(x: isize, y: isize, player: &str);
    pub fn put_den(x: isize, y: isize, player: &str);
    pub fn put_piece(x: isize, y: isize, player: &str, piece: &str);
    pub fn won(player: &str);
    pub fn captured_piece(x: isize, y: isize, owning_player: &str, piece: &str);
    pub fn moved_piece(
        from_x: isize,
        from_y: isize,
        to_x: isize,
        to_y: isize,
        player: &str,
        captures: bool,
    );
}

#[wasm_bindgen]
pub fn new_game() {
    unsafe {
        if !BOARD_PTR.is_null() {
            Box::from_raw(BOARD_PTR);
        }
        BOARD_PTR = Box::into_raw(Box::new(Board::default()));

        clear_board();
        for ((x, y), (g, opt_piece)) in (&*BOARD_PTR).iter() {
            match g {
                Ground::Grass => put_grass(x, y),
                Ground::Trap(owner) => put_trap(x, y, player_to_string(&owner)),
                Ground::Water => put_water(x, y),
                Ground::Den(owner) => put_den(x, y, player_to_string(&owner)),
            }

            if let Some((owner, piece)) = opt_piece {
                put_piece(x, y, player_to_string(&owner), piece_to_string(&piece))
            }
        }
    }
}

fn player_to_string(p: &Player) -> &str {
    match p {
        Player::Player1 => "blue",
        Player::Player2 => "red",
    }
}

fn player_from_string(p: &str) -> Result<Player, &str> {
    match p {
        "red" => Ok(Player::Player2),
        "blue" => Ok(Player::Player1),
        _ => Err("player must be string red or blue."),
    }
}

fn piece_to_string(p: &Piece) -> &str {
    match p {
        Piece::Rat => "rat",
        Piece::Cat => "cat",
        Piece::Dog => "dog",
        Piece::Wolf => "wolf",
        Piece::Leopard => "leopard",
        Piece::Tiger => "tiger",
        Piece::Lion => "lion",
        Piece::Elephant => "elephant",
    }
}

type NextMove = (isize, isize, Vec<(isize, isize)>);
#[wasm_bindgen]
pub fn get_next_moves(player: &str) -> JsValue {
    let b = unsafe { Box::from_raw(BOARD_PTR) };
    let p = match player_from_string(player) {
        Ok(p) => p,
        Err(msg) => return wasm_bindgen::JsValue::from_serde(&Err::<(), &str>(msg)).unwrap(),
    };

    let moves = b.get_next_moves(p);
    unsafe { BOARD_PTR = Box::into_raw(b) };
    wasm_bindgen::JsValue::from_serde(&Ok::<Vec<NextMove>, ()>(
        moves.iter()
            .map(|(_, (x, y), ns)| (*x, *y, ns.iter().map(|(x, y)| (*x, *y)).collect()))
            .collect()
    )).unwrap()
}

fn make_move_internal(
    b: &Board,
    p: Player,
    from_x: isize,
    from_y: isize,
    to_x: isize,
    to_y: isize,
) -> Result<Board, String> {
    match b.make_move(p, (from_x, from_y), (to_x, to_y)) {
        Err(why) => Err(why),
        Ok((new_b, wins, caps)) => {
            if let Some(winner) = wins {
                won(player_to_string(&winner));
            }
            if let Some(cap) = caps {
                captured_piece(
                    to_x,
                    to_y,
                    player_to_string(&cap.0),
                    piece_to_string(&cap.1),
                );
            }
            moved_piece(
                from_x,
                from_y,
                to_x,
                to_y,
                player_to_string(&p),
                caps.is_some(),
            );
            Ok(new_b)
        }
    }
}

#[wasm_bindgen]
pub fn make_ai_move(player: &str, horizon: i32) -> JsValue {
    let p: Player = match player_from_string(player) {
        Ok(p) => p,
        Err(msg) => return wasm_bindgen::JsValue::from_serde(&Err::<(), &str>(msg)).unwrap(),
    };

    let b = unsafe { Box::from_raw(BOARD_PTR) };
    if let Some(((from_x, from_y), (to_x, to_y))) = get_ai_move(&b, p, horizon) {
        match make_move_internal(&b, p, from_x, from_y, to_x, to_y) {
            Err(msg) => {
                unsafe { BOARD_PTR = Box::into_raw(b) };
                wasm_bindgen::JsValue::from_serde(&Err::<(), &str>(&msg)).unwrap()
            }
            Ok(new_b) => {
                unsafe { BOARD_PTR = Box::into_raw(Box::new(new_b)) };
                wasm_bindgen::JsValue::from_serde(&Ok::<(), ()>(())).unwrap()
            }
        }
    } else {
        unsafe { BOARD_PTR = Box::into_raw(b) };
        wasm_bindgen::JsValue::from_serde(&Err::<(), ()>(())).unwrap()
    }
}

#[wasm_bindgen]
pub fn make_move(player: &str, from_x: isize, from_y: isize, to_x: isize, to_y: isize) -> JsValue {
    let p: Player = match player_from_string(player) {
        Ok(p) => p,
        Err(msg) => return wasm_bindgen::JsValue::from_serde(&Err::<(), &str>(msg)).unwrap(),
    };
    let b = unsafe { Box::from_raw(BOARD_PTR) };
    match make_move_internal(&b, p, from_x, from_y, to_x, to_y) {
        Err(msg) => {
            unsafe { BOARD_PTR = Box::into_raw(b) };
            wasm_bindgen::JsValue::from_serde(&Err::<(), &str>(&msg)).unwrap()
        }
        Ok(new_b) => {
            unsafe { BOARD_PTR = Box::into_raw(Box::new(new_b))};
            wasm_bindgen::JsValue::from_serde(&Ok::<(), ()>(())).unwrap()
        }
    }
}
