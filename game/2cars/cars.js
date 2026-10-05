(function () {
    var canvas = document.getElementById('board');
    var ctx = canvas.getContext('2d');
    var scoreEl = document.getElementById('score');
    var statusEl = document.getElementById('status');
    var LANES = [67, 153, 267, 353];
    var PLAYER_Y = 470;
    var PLAYER_H = 52;
    var SPEED = 3;
    var leftLane, rightLane, obstacles, timers, score, alive;

    function roundRect(x, y, w, h, r) {
        ctx.beginPath();
        ctx.moveTo(x + r, y);
        ctx.arcTo(x + w, y, x + w, y + h, r);
        ctx.arcTo(x + w, y + h, x, y + h, r);
        ctx.arcTo(x, y + h, x, y, r);
        ctx.arcTo(x, y, x + w, y, r);
        ctx.closePath();
        ctx.fill();
    }

    function switchLane(side) {
        if (!alive) return;
        if (side === 'left') leftLane = leftLane === 0 ? 1 : 0;
        else rightLane = rightLane === 2 ? 3 : 2;
    }

    function die(reason) {
        alive = false;
        statusEl.textContent = reason + ' Score ' + score + '.';
    }

    function overlaps(o) {
        return o.y < PLAYER_Y + PLAYER_H && o.y + o.h > PLAYER_Y;
    }

    function update() {
        if (!alive) return;
        for (var road = 0; road < 2; road++) {
            timers[road]--;
            if (timers[road] > 0) continue;
            var lane = road * 2 + (Math.random() < 0.5 ? 0 : 1);
            var kind = Math.random() < 0.55 ? 'box' : 'car';
            obstacles.push({
                lane: lane,
                y: -70,
                kind: kind,
                h: kind === 'car' ? 52 : 24
            });
            timers[road] = 95 + Math.floor(Math.random() * 30);
        }
        for (var i = 0; i < obstacles.length; i++) {
            var o = obstacles[i];
            o.y += SPEED;
            var playerLane = o.lane < 2 ? leftLane : rightLane;
            if (o.lane === playerLane && overlaps(o)) {
                if (o.kind === 'car') {
                    die('You hit a car.');
                    return;
                }
                o.collected = true;
                score++;
                scoreEl.textContent = String(score);
            } else if (o.kind === 'box' && !o.collected && o.y > PLAYER_Y + PLAYER_H) {
                die('You missed a box.');
                return;
            }
        }
        obstacles = obstacles.filter(function (o) {
            return !o.collected && o.y < canvas.height + 20;
        });
    }

    function drawCar(lane, color, y) {
        ctx.fillStyle = color;
        roundRect(LANES[lane] - 16, y, 32, PLAYER_H, 8);
        ctx.fillStyle = 'rgba(255,255,255,0.85)';
        ctx.fillRect(LANES[lane] - 10, y + 8, 20, 12);
    }

    function draw() {
        ctx.fillStyle = '#111827';
        ctx.fillRect(0, 0, canvas.width, canvas.height);
        ctx.fillStyle = '#1f2937';
        ctx.fillRect(24, 0, 172, canvas.height);
        ctx.fillRect(224, 0, 172, canvas.height);
        ctx.save();
        ctx.strokeStyle = '#fbbf24';
        ctx.setLineDash([14, 12]);
        ctx.beginPath();
        ctx.moveTo(110, 0);
        ctx.lineTo(110, canvas.height);
        ctx.moveTo(310, 0);
        ctx.lineTo(310, canvas.height);
        ctx.stroke();
        ctx.restore();

        obstacles.forEach(function (o) {
            var x = LANES[o.lane] - 16;
            if (o.kind === 'car') {
                ctx.fillStyle = '#ef4444';
                roundRect(x, o.y, 32, o.h, 8);
            } else {
                ctx.fillStyle = '#fbbf24';
                ctx.fillRect(x + 4, o.y, 24, o.h);
            }
        });

        drawCar(leftLane, '#38bdf8', PLAYER_Y);
        drawCar(rightLane, '#a78bfa', PLAYER_Y);

        if (!alive) {
            ctx.fillStyle = 'rgba(0,0,0,0.55)';
            ctx.fillRect(0, 0, canvas.width, canvas.height);
            ctx.fillStyle = '#fff';
            ctx.font = 'bold 32px sans-serif';
            ctx.textAlign = 'center';
            ctx.fillText('Game over', canvas.width / 2, canvas.height / 2);
            ctx.font = '16px sans-serif';
            ctx.fillText('Score ' + score, canvas.width / 2, canvas.height / 2 + 32);
            ctx.textAlign = 'left';
        }
    }

    function reset() {
        leftLane = 0;
        rightLane = 2;
        obstacles = [];
        timers = [40, 90];
        score = 0;
        alive = true;
        scoreEl.textContent = '0';
        statusEl.textContent = 'Collect the gold boxes. Avoid the red cars.';
    }

    document.addEventListener('keydown', function (event) {
        if (event.repeat || event.metaKey || event.ctrlKey || event.altKey) return;
        if (event.key === 'ArrowLeft' || event.key === 'a' || event.key === 'A') {
            event.preventDefault();
            switchLane('left');
        } else if (event.key === 'ArrowRight' || event.key === 'd' || event.key === 'D') {
            event.preventDefault();
            switchLane('right');
        }
    });
    document.getElementById('switchLeft').addEventListener('click', function () { switchLane('left'); });
    document.getElementById('switchRight').addEventListener('click', function () { switchLane('right'); });
    document.getElementById('restart').addEventListener('click', reset);

    reset();
    function frame() {
        requestAnimationFrame(frame);
        update();
        draw();
    }
    frame();
})();
