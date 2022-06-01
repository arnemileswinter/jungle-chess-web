import "./styles/style.scss"

import init, {new_game, make_move, get_next_moves} from "native"
import {setImportObject} from "native/snippets/native-b653630efb6aa1b2/index"

let nextMoves = [];
let playerToTurn = "blue";
let targets = [];

function fetchMoves(){
    nextMoves = get_next_moves(playerToTurn).Ok;
}

function switchPlayer(){
    if(playerToTurn === "blue"){
        playerToTurn = "red";
    } else if(playerToTurn === "red"){
        playerToTurn = "blue";
    }
}

function tileID(x,y){
    return "tile-"+x+"-"+y;
}

function tileEl(x,y){
    return document.getElementById(tileID(x,y))
}

function tileTarget(x, y, onClick){
    const e = tileEl(x,y);
    const controller = new AbortController();
    e.addEventListener('click', onClick, {once:true,capture:true,signal: controller.signal});

    const img = document.createElement('img');
    img.classList.add("target");
    img.alt = "target";
    img.src = "assets/target.svg";
    e.appendChild(img);

    targets.push({img,abort: () => controller.abort()});
}

function makeMove(fromX,fromY,toX,toY){
    make_move(playerToTurn,fromX,fromY,toX,toY);
    clearTargets();
    switchPlayer();
    fetchMoves();
}

function clearTargets(){
    targets.forEach(t => {
        if (t.img && t.img.parentNode){
            t.img.parentNode.removeChild(t.img);
        }
        t.abort();
    });
    targets = [];
}

function tileClicked(x,y){
    clearTargets();
    nextMoves.filter(m => x===m[0] && y===m[1])
             .flatMap(m => m[2])
             .forEach(m => tileTarget(m[0],
                                      m[1],
                                      () => makeMove(x,y,m[0],m[1])));
}

function newGame(){
    new_game();
    fetchMoves();
}

function main(){
    setImportObject({
        clear(){
            playerToTurn = "blue";
            const root = document.getElementById("board");
            root.innerHTML = "";
            for(let y=0;y<9;++y){
                for(let x=0;x<7;++x){
                    const widget = document.createElement('button');
                    widget.addEventListener('click',() => tileClicked(x,y), {}, true);
                    widget.classList.add('tile');
                    widget.id = tileID(x,y);
                    root.appendChild(widget);
                }
            }
        },
        den(x,y,who) {
            const el = tileEl(x,y);
            el.classList.add("den");
            el.classList.add(who);

            const img = document.createElement("img");
            img.alt = who + " den";
            img.src = "assets/den.svg";
            img.classList.add("den");
            el.appendChild(img);
        },
        trap(x,y,who){
            const el = tileEl(x,y);
            el.classList.add("trap");

            const img = document.createElement("img");
            img.alt = who + " trap";
            img.src = "assets/trap.svg";
            img.classList.add("trap");
            el.appendChild(img);
        },
        piece(x,y,who,piece){
            const el = tileEl(x,y);
            el.classList.add(who);
            el.classList.add("piece");
            el.classList.add(piece);
            el.setAttribute("data-player", who);
            el.setAttribute("data-piece", piece);

            const img = document.createElement("img");
            img.alt = who + " " + piece;
            img.src = "assets/"+piece+".svg";
            img.classList.add("piece");
            el.appendChild(img);
        },
        grass(x,y){
            const el = tileEl(x,y);
            el.classList.add("grass");
        },
        water(x,y){
            const el = tileEl(x,y);
            el.classList.add("water");
        },
        won(who){
            clearTargets();
            const root = document.getElementById("board");
            root.innerHTML = "";
            const title = document.createElement("h1");
            title.innerHTML = "Player " + who + " won!";
            title.classList.add("won");

            const newGameBtn = document.createElement("button");
            newGameBtn.classList.add("new-game");
            newGameBtn.innerHTML = "Rematch";
            newGameBtn.onclick = () => newGame();

            root.appendChild(title);
            root.appendChild(newGameBtn);
        },
        movedPiece(fromX,fromY,toX,toY,who,captured) {
            const oldEl = tileEl(fromX,fromY);

            if(!oldEl){
                return; // new game was started.
            }

            const piece = oldEl.getAttribute("data-piece");
            const movingPlayer = oldEl.getAttribute("data-player");

            oldEl.classList.remove("piece");
            oldEl.classList.remove(piece);
            oldEl.classList.remove(movingPlayer);
            [...oldEl.childNodes]
                .filter(e => e.classList.contains("piece"))
                .forEach(e => oldEl.removeChild(e));

            const newEl = tileEl(toX,toY);

            if(captured){
                const prevPlayer = newEl.getAttribute("data-player");
                const prevPiece = newEl.getAttribute("data-piece");
                newEl.classList.remove(prevPlayer);
                newEl.classList.remove(prevPiece);
                [...newEl.childNodes]
                    .filter(e => e.classList.contains("piece"))
                    .forEach(e => newEl.removeChild(e));
            }

            const img = document.createElement("img");
            img.classList.add("piece");
            img.alt = who + " " + piece;
            img.src = "assets/"+piece+".svg";
            newEl.appendChild(img);

            newEl.setAttribute("data-player", movingPlayer);
            newEl.setAttribute("data-piece", piece);

            newEl.classList.add("piece");
            newEl.classList.add(piece);
            newEl.classList.add(movingPlayer);

            oldEl.removeAttribute("data-player");
            oldEl.removeAttribute("data-piece");
        },
        capturedPiece(x,y,who,piece){
            console.log(x,y,who,piece);
        }
    });
    init().then(() => {
        newGame();
    });
}

main();
