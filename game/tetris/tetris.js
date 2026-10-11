(function () {
    var COLS = 10;
    var ROWS = 20;
    var CELL = 24;
    var BEST_KEY = 'algods-tetris-best';

    var TYPES = {
        I: {
            color: '#22d3ee',
            boxes: [
                [[0, 1], [1, 1], [2, 1], [3, 1]],
                [[2, 0], [2, 1], [2, 2], [2, 3]],
                [[0, 2], [1, 2], [2, 2], [3, 2]],
                [[1, 0], [1, 1], [1, 2], [1, 3]]
            ]
        },
        O: {
            color: '#facc15',
            boxes: [
                [[1, 0], [2, 0], [1, 1], [2, 1]],
                [[1, 0], [2, 0], [1, 1], [2, 1]],
                [[1, 0], [2, 0], [1, 1], [2, 1]],
                [[1, 0], [2, 0], [1, 1], [2, 1]]
            ]
        },
        T: {
            color: '#c084fc',
            boxes: [
                [[1, 0], [0, 1], [1, 1], [2, 1]],
                [[1, 0], [1, 1], [2, 1], [1, 2]],
                [[0, 1], [1, 1], [2, 1], [1, 2]],
                [[1, 0], [0, 1], [1, 1], [1, 2]]
            ]
        },
        S: {
            color: '#4ade80',
            boxes: [
                [[1, 0], [2, 0], [0, 1], [1, 1]],
                [[1, 0], [1, 1], [2, 1], [2, 2]],
                [[1, 1], [2, 1], [0, 2], [1, 2]],
                [[0, 0], [0, 1], [1, 1], [1, 2]]
            ]
        },
        Z: {
            color: '#fb7185',
            boxes: [
                [[0, 0], [1, 0], [1, 1], [2, 1]],
                [[2, 0], [1, 1], [2, 1], [1, 2]],
                [[0, 1], [1, 1], [1, 2], [2, 2]],
                [[1, 0], [0, 1], [1, 1], [0, 2]]
            ]
        },
        J: {
            color: '#60a5fa',
            boxes: [
                [[0, 0], [0, 1], [1, 1], [2, 1]],
                [[1, 0], [2, 0], [1, 1], [1, 2]],
                [[0, 1], [1, 1], [2, 1], [2, 2]],
                [[1, 0], [1, 1], [0, 2], [1, 2]]
            ]
        },
        L: {
            color: '#fb923c',
            boxes: [
                [[2, 0], [0, 1], [1, 1], [2, 1]],
                [[1, 0], [1, 1], [1, 2], [2, 2]],
                [[0, 1], [1, 1], [2, 1], [0, 2]],
                [[0, 0], [1, 0], [1, 1], [1, 2]]
            ]
        }
    };

    // Wall kicks with y flipped so positive is down the screen.
    var KICKS = {
        normal: {
            '0>1': [[0, 0], [-1, 0], [-1, -1], [0, 2], [-1, 2]],
            '1>0': [[0, 0], [1, 0], [1, 1], [0, -2], [1, -2]],
            '1>2': [[0, 0], [1, 0], [1, 1], [0, -2], [1, -2]],
            '2>1': [[0, 0], [-1, 0], [-1, -1], [0, 2], [-1, 2]],
            '2>3': [[0, 0], [1, 0], [1, -1], [0, 2], [1, 2]],
            '3>2': [[0, 0], [-1, 0], [-1, 1], [0, -2], [-1, -2]],
            '3>0': [[0, 0], [-1, 0], [-1, 1], [0, -2], [-1, -2]],
            '0>3': [[0, 0], [1, 0], [1, -1], [0, 2], [1, 2]]
        },
        I: {
            '0>1': [[0, 0], [-2, 0], [1, 0], [-2, 1], [1, -2]],
            '1>0': [[0, 0], [2, 0], [-1, 0], [2, -1], [-1, 2]],
            '1>2': [[0, 0], [-1, 0], [2, 0], [-1, -2], [2, 1]],
            '2>1': [[0, 0], [1, 0], [-2, 0], [1, 2], [-2, -1]],
            '2>3': [[0, 0], [2, 0], [-1, 0], [2, -1], [-1, 2]],
            '3>2': [[0, 0], [-2, 0], [1, 0], [-2, 1], [1, -2]],
            '3>0': [[0, 0], [1, 0], [-2, 0], [1, 2], [-2, -1]],
            '0>3': [[0, 0], [-1, 0], [2, 0], [-1, -2], [2, 1]]
        }
    };

    var NAMES = ['I', 'O', 'T', 'S', 'Z', 'J', 'L'];
    var LINE_SCORE = [0, 100, 300, 500, 800];

    var boardCanvas = document.getElementById('board');
    var nextCanvas = document.getElementById('next');
    var holdCanvas = document.getElementById('hold');
    var ctx = boardCanvas.getContext('2d');
    var nextCtx = nextCanvas.getContext('2d');
    var holdCtx = holdCanvas.getContext('2d');
    var scoreEl = document.getElementById('score');
    var bestEl = document.getElementById('best');
    var levelEl = document.getElementById('level');
    var linesEl = document.getElementById('lines');
    var statusEl = document.getElementById('status');

    var board, bag, piece, nextType, holdType, heldUsed;
    var score, best, lines, level, alive, paused;
    var gravity, lockTimer, lockResets, locking;
    var last = 0;
    var repeat = { dir: 0, wait: 0, down: false, downWait: 0 };

    function readBest() {
        try { return Number(localStorage.getItem(BEST_KEY)) || 0; }
        catch (err) { return 0; }
    }

    function writeBest(value) {
        try { localStorage.setItem(BEST_KEY, String(value)); }
        catch (err) { /* private mode */ }
    }

    function emptyRow() {
        var row = [];
        for (var x = 0; x < COLS; x++) row.push(null);
        return row;
    }

    function shuffle(list) {
        for (var i = list.length - 1; i > 0; i--) {
            var j = Math.floor(Math.random() * (i + 1));
            var tmp = list[i];
            list[i] = list[j];
            list[j] = tmp;
        }
        return list;
    }

    function pull() {
        if (bag.length === 0) bag = shuffle(NAMES.slice());
        return bag.shift();
    }

    function cellsOf(p) {
        return TYPES[p.type].boxes[p.r].map(function (b) {
            return { x: p.x + b[0], y: p.y + b[1] };
        });
    }

    function fits(p) {
        var list = cellsOf(p);
        for (var i = 0; i < list.length; i++) {
            var c = list[i];
            if (c.x < 0 || c.x >= COLS || c.y >= ROWS) return false;
            if (c.y >= 0 && board[c.y][c.x]) return false;
        }
        return true;
    }

    function make(type) {
        return { type: type, r: 0, x: 3, y: 0 };
    }

    function grounded(p) {
        return !fits({ type: p.type, r: p.r, x: p.x, y: p.y + 1 });
    }

    function ghostY() {
        var y = piece.y;
        while (fits({ type: piece.type, r: piece.r, x: piece.x, y: y + 1 })) y += 1;
        return y;
    }

    function addScore(n) {
        score += n;
        scoreEl.textContent = String(score);
        if (score > best) {
            best = score;
            bestEl.textContent = String(best);
            writeBest(best);
        }
    }

    function say(text) {
        statusEl.textContent = text;
    }

    function spawn(type) {
        piece = make(type);
        locking = false;
        lockTimer = 0;
        lockResets = 0;
        gravity = 0;
        if (!fits(piece)) endGame();
    }

    function spawnNext() {
        var type = nextType;
        nextType = pull();
        heldUsed = false;
        spawn(type);
    }

    function interval() {
        return Math.max(70, Math.round(800 * Math.pow(0.82, level - 1)));
    }

    function noteLanding() {
        if (grounded(piece)) {
            if (!locking) {
                locking = true;
                lockTimer = 0;
            }
        } else {
            locking = false;
            lockTimer = 0;
        }
    }

    function shift(dx) {
        if (!alive || paused) return false;
        var trial = { type: piece.type, r: piece.r, x: piece.x + dx, y: piece.y };
        if (!fits(trial)) return false;
        piece = trial;
        if (locking && lockResets < 12) {
            lockTimer = 0;
            lockResets += 1;
        }
        noteLanding();
        return true;
    }

    function rotate(dir) {
        if (!alive || paused) return false;
        var next = (piece.r + dir + 4) % 4;
        var kicks = piece.type === 'O' ? [[0, 0]] : (piece.type === 'I' ? KICKS.I : KICKS.normal)[piece.r + '>' + next];
        for (var i = 0; i < kicks.length; i++) {
            var trial = {
                type: piece.type,
                r: next,
                x: piece.x + kicks[i][0],
                y: piece.y + kicks[i][1]
            };
            if (fits(trial)) {
                piece = trial;
                if (locking && lockResets < 12) {
                    lockTimer = 0;
                    lockResets += 1;
                }
                noteLanding();
                return true;
            }
        }
        return false;
    }

    function lock() {
        cellsOf(piece).forEach(function (c) {
            if (c.y >= 0 && c.y < ROWS) board[c.y][c.x] = TYPES[piece.type].color;
        });
        var cleared = 0;
        for (var y = ROWS - 1; y >= 0; y--) {
            var full = true;
            for (var x = 0; x < COLS; x++) {
                if (!board[y][x]) { full = false; break; }
            }
            if (full) {
                board.splice(y, 1);
                board.unshift(emptyRow());
                cleared += 1;
                y += 1;
            }
        }
        if (cleared) {
            addScore(LINE_SCORE[cleared] * level);
            lines += cleared;
            linesEl.textContent = String(lines);
            var nextLevel = Math.floor(lines / 10) + 1;
            if (nextLevel !== level) {
                level = nextLevel;
                levelEl.textContent = String(level);
                say('Level ' + level + '.');
            } else if (cleared === 4) {
                say('Tetris.');
            } else if (cleared > 1) {
                say(cleared + ' lines.');
            }
        }
        spawnNext();
    }

    function fall(scoring) {
        if (!alive || paused) return false;
        if (!fits({ type: piece.type, r: piece.r, x: piece.x, y: piece.y + 1 })) {
            noteLanding();
            return false;
        }
        piece.y += 1;
        if (scoring) addScore(1);
        gravity = 0;
        noteLanding();
        return true;
    }

    function softDrop() {
        return fall(true);
    }

    function hardDrop() {
        if (!alive || paused) return;
        var dist = ghostY() - piece.y;
        piece.y += dist;
        if (dist) addScore(dist * 2);
        lock();
    }

    function hold() {
        if (!alive || paused || heldUsed) return;
        heldUsed = true;
        var current = piece.type;
        if (holdType) {
            var saved = holdType;
            holdType = current;
            spawn(saved);
        } else {
            holdType = current;
            spawnNext();
            heldUsed = true;
        }
    }

    function endGame() {
        alive = false;
        paused = false;
        say('Game over. ' + score + ' points.');
    }

    function roundRect(context, x, y, w, h, r) {
        context.beginPath();
        context.moveTo(x + r, y);
        context.arcTo(x + w, y, x + w, y + h, r);
        context.arcTo(x + w, y + h, x, y + h, r);
        context.arcTo(x, y + h, x, y, r);
        context.arcTo(x, y, x + w, y, r);
        context.closePath();
    }

    function paintCell(context, x, y, size, color, ghost) {
        var pad = Math.max(1, size * 0.08);
        roundRect(context, x + pad, y + pad, size - pad * 2, size - pad * 2, Math.max(2, size * 0.18));
        if (ghost) {
            context.strokeStyle = color;
            context.lineWidth = Math.max(1, size * 0.08);
            context.stroke();
            return;
        }
        context.fillStyle = color;
        context.fill();
        context.fillStyle = 'rgba(255,255,255,0.35)';
        roundRect(context, x + pad + size * 0.12, y + pad + size * 0.1, size * 0.38, size * 0.16, size * 0.08);
        context.fill();
    }

    function paintPreview(context, canvas, type, dim) {
        context.clearRect(0, 0, canvas.width, canvas.height);
        context.fillStyle = '#1b1522';
        roundRect(context, 0, 0, canvas.width, canvas.height, 12);
        context.fill();
        if (!type) return;
        var boxes = TYPES[type].boxes[0];
        var minX = 4, minY = 4, maxX = 0, maxY = 0;
        boxes.forEach(function (b) {
            if (b[0] < minX) minX = b[0];
            if (b[1] < minY) minY = b[1];
            if (b[0] > maxX) maxX = b[0];
            if (b[1] > maxY) maxY = b[1];
        });
        var size = 16;
        var w = (maxX - minX + 1) * size;
        var h = (maxY - minY + 1) * size;
        var ox = (canvas.width - w) / 2 - minX * size;
        var oy = (canvas.height - h) / 2 - minY * size;
        context.globalAlpha = dim ? 0.35 : 1;
        boxes.forEach(function (b) {
            paintCell(context, ox + b[0] * size, oy + b[1] * size, size, TYPES[type].color, false);
        });
        context.globalAlpha = 1;
    }

    function draw() {
        ctx.fillStyle = '#1b1522';
        ctx.fillRect(0, 0, boardCanvas.width, boardCanvas.height);
        ctx.strokeStyle = 'rgba(255,255,255,0.05)';
        ctx.lineWidth = 1;
        for (var x = 1; x < COLS; x++) {
            ctx.beginPath();
            ctx.moveTo(x * CELL + 0.5, 0);
            ctx.lineTo(x * CELL + 0.5, ROWS * CELL);
            ctx.stroke();
        }
        for (var y = 1; y < ROWS; y++) {
            ctx.beginPath();
            ctx.moveTo(0, y * CELL + 0.5);
            ctx.lineTo(COLS * CELL, y * CELL + 0.5);
            ctx.stroke();
        }
        for (var row = 0; row < ROWS; row++) {
            for (var col = 0; col < COLS; col++) {
                if (board[row][col]) paintCell(ctx, col * CELL, row * CELL, CELL, board[row][col], false);
            }
        }
        if (piece) {
            if (alive) {
                var gy = ghostY();
                if (gy !== piece.y) {
                    cellsOf({ type: piece.type, r: piece.r, x: piece.x, y: gy }).forEach(function (c) {
                        if (c.y >= 0) paintCell(ctx, c.x * CELL, c.y * CELL, CELL, TYPES[piece.type].color, true);
                    });
                }
            }
            cellsOf(piece).forEach(function (c) {
                if (c.y >= 0) paintCell(ctx, c.x * CELL, c.y * CELL, CELL, TYPES[piece.type].color, false);
            });
        }
        if (paused) {
            ctx.fillStyle = 'rgba(27, 21, 34, 0.72)';
            ctx.fillRect(0, 0, boardCanvas.width, boardCanvas.height);
            ctx.fillStyle = '#f6e6c8';
            ctx.font = '800 28px Outfit, sans-serif';
            ctx.textAlign = 'center';
            ctx.textBaseline = 'middle';
            ctx.fillText('Paused', boardCanvas.width / 2, boardCanvas.height / 2);
        }
        paintPreview(nextCtx, nextCanvas, nextType, false);
        paintPreview(holdCtx, holdCanvas, holdType, heldUsed);
    }

    function reset() {
        board = [];
        for (var y = 0; y < ROWS; y++) board.push(emptyRow());
        bag = [];
        piece = null;
        holdType = null;
        heldUsed = false;
        score = 0;
        lines = 0;
        level = 1;
        alive = true;
        paused = false;
        gravity = 0;
        locking = false;
        lockTimer = 0;
        lockResets = 0;
        repeat.dir = 0;
        repeat.wait = 0;
        repeat.down = false;
        repeat.downWait = 0;
        scoreEl.textContent = '0';
        linesEl.textContent = '0';
        levelEl.textContent = '1';
        nextType = pull();
        say('Arrows move. Up rotates. Space drops.');
        spawn(pull());
        draw();
    }

    function togglePause() {
        if (!alive) return;
        paused = !paused;
        say(paused ? 'Paused.' : 'Back in.');
        draw();
    }

    function loop(now) {
        if (!last) last = now;
        var dt = Math.min(50, now - last);
        last = now;
        if (alive && !paused && piece) {
            if (repeat.dir) {
                repeat.wait -= dt;
                if (repeat.wait <= 0) {
                    if (shift(repeat.dir)) repeat.wait = 45;
                    else repeat.wait = 45;
                }
            }
            if (repeat.down) {
                repeat.downWait -= dt;
                if (repeat.downWait <= 0) {
                    fall(true);
                    repeat.downWait = 45;
                }
            }
            gravity += dt;
            if (gravity >= interval()) {
                gravity = 0;
                fall(false);
            }
            if (locking) {
                lockTimer += dt;
                if (lockTimer >= 500) lock();
            }
        }
        draw();
        requestAnimationFrame(loop);
    }

    document.addEventListener('keydown', function (event) {
        if (event.metaKey || event.ctrlKey || event.altKey) return;
        var key = event.key;
        if (['ArrowLeft', 'ArrowRight', 'ArrowDown', 'ArrowUp', ' ', 'Spacebar'].indexOf(key) !== -1) {
            event.preventDefault();
        }
        if (event.repeat) return;
        if (key === 'p' || key === 'P' || key === 'Escape') {
            togglePause();
            return;
        }
        if (key === 'r' || key === 'R') {
            if (!alive) reset();
            return;
        }
        if (!alive || paused) return;
        if (key === 'ArrowLeft') {
            shift(-1);
            repeat.dir = -1;
            repeat.wait = 160;
        } else if (key === 'ArrowRight') {
            shift(1);
            repeat.dir = 1;
            repeat.wait = 160;
        } else if (key === 'ArrowDown') {
            softDrop();
            repeat.down = true;
            repeat.downWait = 160;
        } else if (key === 'ArrowUp' || key === 'x' || key === 'X') {
            rotate(1);
        } else if (key === 'z' || key === 'Z') {
            rotate(-1);
        } else if (key === ' ' || key === 'Spacebar') {
            hardDrop();
        } else if (key === 'c' || key === 'C' || key === 'Shift') {
            hold();
        }
    });

    document.addEventListener('keyup', function (event) {
        if (event.key === 'ArrowLeft' && repeat.dir === -1) repeat.dir = 0;
        if (event.key === 'ArrowRight' && repeat.dir === 1) repeat.dir = 0;
        if (event.key === 'ArrowDown') repeat.down = false;
    });

    document.querySelectorAll('[data-act]').forEach(function (button) {
        button.addEventListener('click', function () {
            var act = button.getAttribute('data-act');
            if (act === 'left') shift(-1);
            else if (act === 'right') shift(1);
            else if (act === 'soft') softDrop();
            else if (act === 'hard') hardDrop();
            else if (act === 'cw') rotate(1);
            else if (act === 'ccw') rotate(-1);
            else if (act === 'hold') hold();
        });
    });

    var startX = 0;
    var startY = 0;
    var swiped = false;
    boardCanvas.addEventListener('pointerdown', function (event) {
        startX = event.clientX;
        startY = event.clientY;
        swiped = false;
    });
    boardCanvas.addEventListener('pointerup', function (event) {
        var dx = event.clientX - startX;
        var dy = event.clientY - startY;
        if (Math.hypot(dx, dy) < 24) {
            if (!swiped) rotate(1);
            return;
        }
        swiped = true;
        if (Math.abs(dx) > Math.abs(dy)) shift(dx > 0 ? 1 : -1);
        else if (dy > 0) hardDrop();
        else rotate(1);
    });

    document.getElementById('restart').addEventListener('click', reset);
    document.getElementById('pause').addEventListener('click', togglePause);

    best = readBest();
    bestEl.textContent = String(best);
    reset();
    requestAnimationFrame(loop);
})();
