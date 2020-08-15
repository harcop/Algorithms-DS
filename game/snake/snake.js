var w = 40;
var rows, cols
var grids = [];
var pos = Math.floor(Math.random() * 10) + 1;

var dir = 'down';
var path = [10, 11, 12];
var road = [];
var snake = null;
function setup() {
    createCanvas(400, 400);
    rows = floor(height/w);
    cols = floor(width/w);
    
    for(let y = 0; y < rows; y++) {
        for(let x = 0; x < cols; x++) {
            let ceil = new Ceil(x, y);
            grids.push(ceil);
        }
    }
    snake = new Snake();
    snake.append(15).append(14).append(13).append(12);
    console.log(snake);
}

function draw() {
    background(33, 56, 89); 
    for (let i = 0; i < grids.length; i++) {
        grids[i].show();
    }
    
//    snake.traverse();
//    console.log(snake) 
    frameRate(5);
}