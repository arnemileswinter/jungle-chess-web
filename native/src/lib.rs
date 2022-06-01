use junglechess::board::{Board, Piece, Player, Ground};
use serde::Serialize;
use std::ptr;
use wasm_bindgen::prelude::*;

static mut BOARD_PTR: *mut Board = ptr::null_mut();

#[wasm_bindgen(module="/index.js")]
extern "C" {
    pub fn clear_board();
    pub fn put_grass(x : i32,y : i32);
    pub fn put_water(x:i32,y:i32);
    pub fn put_trap(x:i32,y:i32,player:&str);
    pub fn put_den(x:i32,y:i32,player:&str);
    pub fn put_piece(x : i32, y : i32, player : &str, piece: &str);
    pub fn won(player:&str);
    pub fn captured_piece(x:i32,y:i32, owning_player:&str, piece:&str);
    pub fn moved_piece(from_x:i32,from_y:i32,to_x:i32,to_y:i32,player:&str,captures:bool);
}


#[wasm_bindgen]
pub fn new_game() {
    unsafe {
        if !BOARD_PTR.is_null() {
            Box::from_raw(BOARD_PTR);
        }
        BOARD_PTR = Box::into_raw(Box::new(Board::default()));

        clear_board();
        for ((x,y),(g,opt_piece)) in (&*BOARD_PTR).iter() {
            match g {
                Ground::Grass => put_grass(x as i32,y as i32),
                Ground::Trap(owner) => put_trap(x as i32, y as i32, player_to_string(&owner)),
                Ground::Water => put_water(x as i32, y as i32),
                Ground::Den(owner) => put_den(x as i32, y as i32, player_to_string(&owner)),
            }

            for (owner,piece) in opt_piece {
                put_piece(x as i32, y as i32, player_to_string(&owner), piece_to_string(&piece))
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

#[derive(Serialize)]
pub struct GameMove {
    pub winner: Option<String>,
    pub captured: Option<(String, String)>,
}

#[derive(Serialize)]
pub enum GameMoveResult {
    Ok,
    Err(String),
}

type NextMove = (i32,i32,Vec<(i32,i32)>);

#[derive(Serialize)]
pub enum NextMoveResult {
    Ok(Vec<NextMove>),
    Err(String)
}

#[wasm_bindgen]
pub fn get_next_moves(player: &JsValue) -> JsValue{
    let b = unsafe {Box::from_raw(BOARD_PTR)};
    let mut p = Player::Player1;
    match player.as_string() {
        Some(s) => {
            if s == "red" {
                p = Player::Player2
            } else if s != "blue" {
                return wasm_bindgen::JsValue::from_serde(&NextMoveResult::Err(
                    "player must be red or blue.".to_string(),
                ))
                .unwrap();
            }
        }
        _ => {
            unsafe { BOARD_PTR = Box::into_raw(b) };
            return wasm_bindgen::JsValue::from_serde(&NextMoveResult::Err(
                "player must be string red or blue.".to_string(),
            ))
            .unwrap();
        }
    }

    let moves = b.get_next_moves(p);
    unsafe { BOARD_PTR = Box::into_raw(b) };
    wasm_bindgen::JsValue::from_serde(
        &NextMoveResult::Ok(
            moves.iter()
                 .map(|(_,(x,y),ns)| (*x as i32,
                                      *y as i32,
                                      ns.iter()
                                        .map(|(x,y)| (*x as i32, *y as i32))
                                        .collect())
                 ).collect()
        )).unwrap()
}

#[wasm_bindgen]
pub fn make_move(
    player: &JsValue,
    from_x: &JsValue,
    from_y: &JsValue,
    to_x: &JsValue,
    to_y: &JsValue,
) -> JsValue {
    let b = unsafe { Box::from_raw(BOARD_PTR) };
    let mut p: Player = Player::Player1;
    match player.as_string() {
        Some(s) => {
            if s == "red" {
                p = Player::Player2
            } else if s != "blue" {
                return wasm_bindgen::JsValue::from_serde(&GameMoveResult::Err(
                    "player must be red or blue.".to_string(),
                ))
                .unwrap();
            }
        }
        _ => {
            return wasm_bindgen::JsValue::from_serde(&GameMoveResult::Err(
                "player must be string red or blue.".to_string(),
            ))
            .unwrap();
        }
    }
    for opt in [from_x, from_y, to_x, to_y] {
        if opt.as_f64().is_none() {
            return wasm_bindgen::JsValue::from_serde(&GameMoveResult::Err(
                "from_x, from_y, to_x, to_y are expected to be numbers.".to_string(),
            ))
            .unwrap();
        }
    }

    let res = b.make_move(
        p,
        (
            from_x.as_f64().unwrap() as isize,
            from_y.as_f64().unwrap() as isize,
        ),
        (
            to_x.as_f64().unwrap() as isize,
            to_y.as_f64().unwrap() as isize,
        ),
    );
    match res {
        Err(why) => {
            unsafe { BOARD_PTR = Box::into_raw(b) };
            wasm_bindgen::JsValue::from_serde(&GameMoveResult::Err(why)).unwrap()
        }
        Ok((new_b, wins, caps)) => {
            unsafe {
                BOARD_PTR = Box::into_raw(Box::new(new_b));
            }
            if let Some(winner) = wins {
                won(player_to_string(&winner));
            }
            if let Some(cap) = caps {
                captured_piece(to_x.as_f64().unwrap() as i32, to_y.as_f64().unwrap() as i32, player_to_string(&cap.0), piece_to_string(&cap.1));
            }
            moved_piece(from_x.as_f64().unwrap() as i32,
                       from_y.as_f64().unwrap() as i32,
                       to_x.as_f64().unwrap() as i32,
                       to_y.as_f64().unwrap() as i32,
                       player_to_string(&p),
                       caps.is_some()
                       );
            wasm_bindgen::JsValue::from_serde(&GameMoveResult::Ok).unwrap()
        }
    }
}
