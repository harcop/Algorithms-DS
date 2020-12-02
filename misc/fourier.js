let time = 0;
let wave = [];
function setup () {
    createCanvas(600,400);
}

function draw() {
    background(0);
    
    translate(200, 200);
    let radius = 100;
    noFill()
    stroke(255);
    ellipse(0, 0, radius * 2);
    
    
    
    let sRadius = 4;
    let x = radius * cos(time);
    let y = radius * sin(time);
    
    noFill()
    ellipse(x,y,32);
    
    fill(255)
    wave.unshift(y);
    line(0, 0, x, y)
    ellipse(x, y, sRadius * 2);
    translate(200, 0)
    
    line(x-200, y, 0, wave[0]);
    beginShape();
    noFill();
    for(let i =0; i< wave.length; i++) {
        vertex(i,wave[i]);
    }
    endShape();
    
    if (wave.length > 200) {
        wave.pop();
    }
    time += 0.05;
}