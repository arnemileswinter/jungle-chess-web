let importObject = {}

function assert(what) {
  if (!what){
    throw new Error("assertion failed");
  }
}

export function setImportObject(obj){
  assert(obj.den);
  assert(obj.trap);
  assert(obj.piece);
  assert(obj.clear);
  assert(obj.grass);
  assert(obj.water);
  assert(obj.won);
  assert(obj.movedPiece);
  assert(obj.capturedPiece);
  importObject = obj;
}

export function put_den(x,y,player){
  importObject.den(x,y,player);
}

export function put_trap(x,y,player) {
  importObject.trap(x,y,player);
}

export function put_piece(x,y,player,piece) {
  importObject.piece(x,y,player,piece);
}

export function clear_board(){
  importObject.clear();
}

export function put_grass(x,y){
  importObject.grass(x,y);
}

export function put_water(x,y){
  importObject.water(x,y);
}

export function won(who){
  importObject.won(who);
}

export function captured_piece(x,y,owner,piece){
  importObject.capturedPiece(x,y,owner,piece);
}

export function moved_piece(fromX,fromY,toX,toY,owner,captures){
  importObject.movedPiece(fromX,fromY,toX,toY,owner,captures);
}
