(function () {
    var canvas = document.getElementById('board');
    var ctx = canvas.getContext('2d');
    var scoreEl = document.getElementById('score');
    var statusEl = document.getElementById('status');
    var TILE = 20;
    var COLS = canvas.width / TILE;
    var ROWS = canvas.height / TILE;
    var snake, dir, queued, food, score, alive, timer;

    function rand(n) {
        return Math.floor(Math.random() * n);
    }

    function placeFood() {
        var open = [];
        for (var y = 0; y < ROWS; y++) {
            for (var x = 0; x < COLS; x++) {
                var taken = snake.some(function (part) { return part.x === x && part.y === y; });
                if (!taken) open.push({ x: x, y: y });
            }
        }
        food = open[rand(open.length)];
    }

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

    function draw() {
        ctx.fillStyle = '#10131b';
        ctx.fillRect(0, 0, canvas.width, canvas.height);
        ctx.fillStyle = '#fb7185';
        ctx.beginPath();
        ctx.arc(food.x * TILE + TILE / 2, food.y * TILE + TILE / 2, TILE / 2 - 3, 0, Math.PI * 2);
        ctx.fill();
        snake.forEach(function (part, index) {
            ctx.fillStyle = index === 0 ? '#86efac' : '#22c55e';
            roundRect(part.x * TILE + 1, part.y * TILE + 1, TILE - 2, TILE - 2, 4);
        });
    }

    function stop(message) {
        alive = false;
        clearInterval(timer);
        statusEl.textContent = message;
        draw();
    }

    function tick() {
        if (!alive) return;
        dir = queued;
        var head = { x: snake[0].x + dir.x, y: snake[0].y + dir.y };
        if (head.x < 0 || head.y < 0 || head.x >= COLS || head.y >= ROWS) {
            stop('Game over. You hit a wall.');
            return;
        }
        var growing = food && head.x === food.x && head.y === food.y;
        var body = growing ? snake : snake.slice(0, -1);
        if (body.some(function (part) { return part.x === head.x && part.y === head.y; })) {
            stop('Game over. You hit your tail.');
            return;
        }
        snake.unshift(head);
        if (growing) {
            score += 1;
            scoreEl.textContent = String(score);
            if (snake.length === COLS * ROWS) {
                stop('You filled the board.');
                return;
            }
            placeFood();
        } else {
            snake.pop();
        }
        draw();
    }

    function setDir(x, y) {
        if (!alive) return;
        if (x === -dir.x && y === -dir.y) return;
        if (x === -queued.x && y === -queued.y) return;
        queued = { x: x, y: y };
    }

    function reset() {
        snake = [{ x: 6, y: 10 }, { x: 5, y: 10 }, { x: 4, y: 10 }];
        dir = { x: 1, y: 0 };
        queued = { x: 1, y: 0 };
        score = 0;
        alive = true;
        scoreEl.textContent = '0';
        statusEl.textContent = 'Arrow keys to turn.';
        placeFood();
        draw();
        clearInterval(timer);
        timer = setInterval(tick, 120);
    }

    document.addEventListener('keydown', function (event) {
        if (event.metaKey || event.ctrlKey || event.altKey) return;
        var map = {
            ArrowUp: [0, -1],
            ArrowDown: [0, 1],
            ArrowLeft: [-1, 0],
            ArrowRight: [1, 0]
        };
        if (!map[event.key]) return;
        event.preventDefault();
        setDir(map[event.key][0], map[event.key][1]);
    });

    var startX = 0;
    var startY = 0;
    canvas.addEventListener('pointerdown', function (event) {
        startX = event.clientX;
        startY = event.clientY;
    });
    canvas.addEventListener('pointerup', function (event) {
        var dx = event.clientX - startX;
        var dy = event.clientY - startY;
        if (Math.hypot(dx, dy) < 20) return;
        if (Math.abs(dx) > Math.abs(dy)) setDir(dx > 0 ? 1 : -1, 0);
        else setDir(0, dy > 0 ? 1 : -1);
    });

    document.getElementById('restart').addEventListener('click', reset);
    reset();
})();
