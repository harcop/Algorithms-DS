var w = 25;
var rows, cols
var carH = 40;
var carW = 20;
var boxH = 20;
var line1 = [];
var line2 = [];
var line3 = [];
var line4 = [];
var cars =  [];
function setup() {
    createCanvas(400, 400);
    rows = floor(height/w);
    cols = floor(width/w);
    var line1 = new Box(40, line1);
//    var line2 = new Car(140, line2);
//    var line3 = new Box(240, line3);
    var line4 = new Car(340, line4);
//    cars = [line1, line2, line3, line4];
    cars = [line1, line4];
}

function draw() {
    background(40,30,90);
    centerLine()
    cars.forEach(car => {
        car.show();
    })
}

function Car(x, side) {
    this.x = x;
    this.y = floor(random() * - 20) - 0; 
    this.side = side;
//    this.y = -10; 
    this.show = function () {
        stroke(200);
        fill(0,250,250);
        rect(this.x,this.y,carW, carH);
        this.y++;
        if (this.y >= height-10) {
            cars.splice(0,1);
            let rand = floor(random() * 2);
            console.log(rand);
            if (rand === 1) {
                car = new Box(this.x, this.side);
            }else {
                car = new Car(this.x, this.side);
            }
            cars.push(car);
            
        }
//        rect(140,0,carW, carH);
//        rect(240,0,carW, carH);
//        rect(340,0,carW, carH);
    }
}

function Box(x, side) {
    this.x = x;
    this.y = floor(random() * - 20) - 0;
    this.side = side;
//    this.y = -10; 
    this.show = function () {
        stroke(200);
        fill(250,121,0);
        rect(this.x,this.y,carW, boxH);
        this.y++;
        if (this.y >= height-10) {
            cars.splice(0,1);
            let rand = floor(random() * 2);
            if (rand === 1) {
                car = new Box(this.x, this.side);
            }else {
                car = new Car(this.x, this.side);
            }
            cars.push(car);
        }
//        rect(140,0,carW, carH);
//        rect(240,0,carW, carH);
//        rect(340,0,carW, carH);
    }
}




function centerLine() {
    let s = width/2;
    let t = width/4;
    let u = width*0.75;
    rect(s,0,2,height);
    rect(t,0,1,height);
    rect(u,0,1,height);
}
