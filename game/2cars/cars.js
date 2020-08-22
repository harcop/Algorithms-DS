var w = 25;
var rows, cols
var carH = 40;
var carW = 20;
var boxH = 20;
var line1 = [];
var line2 = [];
var line3 = [];
var line4 = [];

var cars = {
    line1: line1,
    line2: line2,
    line3: line3,
    line4: line4,
};
function setup() {
    console.log(cars["line1"]);
    createCanvas(400, 400);
    rows = floor(height/w);
    cols = floor(width/w);
    line1[0] = new Box(40, "line1");
    line3[0] = new Box(240, "line3");
}

function draw() {
    background(40,30,90);
    centerLine()

    line1.forEach(car => {
        car.show();
    })
//    line2.forEach(car => {
//        car.show();
//    })
    line3.forEach(car => {
        car.show();
    })
//    line4.forEach(car => {
//        car.show();
//    })
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
        this.y = this.y + 5;
        if (this.y >= height/2 + 3 && this.y < height/2 + 8) {
            let rand = floor(random() * 2);
            let rand2 = floor(random() * 2);
            let newX = this.x;
            if (rand2 === 1) {
                if(this.x === 40) {
                    newX = 140;
                }
                else if(this.x == 140) {
                    newX = 40
                }
                else if(this.x == 240) {
                    newX = 340
                }
                else if(this.x == 340) {
                    newX = 240
                }
            }
            if (rand === 1) {
                car = new Box(newX, this.side);
            }else {
                car = new Car(newX, this.side);
            }
            cars[side].push(car);   
        }
        if (this.y > height - 5) {
            cars[side].splice(0,1);
        }
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
        this.y = this.y + 5;
        if (this.y >= height/2 + 3 && this.y < height/2 + 8)  {
            let rand = floor(random() * 2);
            let rand2 = floor(random() * 2);
            let newX = this.x;
            if (rand2 === 1) {
                if(this.x === 40) {
                    newX = 140;
                }
                else if(this.x == 140) {
                    newX = 40
                }
                else if(this.x == 240) {
                    newX = 340
                }
                else if(this.x == 340) {
                    newX = 240
                }
            }
            if (rand === 1) {
                car = new Box(newX, this.side);
            }else {
                car = new Car(newX, this.side);
            }
            cars[side].push(car);   
        }
        if (this.y > height - 5) {
            cars[side].splice(0,1);
        }
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
