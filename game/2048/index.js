function slide(values) {
    const tiles = values.filter(function (v) { return v !== 0; });
    const out = [];
    let gained = 0;
    for (let i = 0; i < tiles.length; i++) {
        if (i + 1 < tiles.length && tiles[i] === tiles[i + 1]) {
            const merged = tiles[i] * 2;
            out.push(merged);
            gained += merged;
            i++;
        } else {
            out.push(tiles[i]);
        }
    }
    while (out.length < values.length) out.push(0);
    return { line: out, gained: gained };
}

function linesFor(dir) {
    const lines = [];
    for (let i = 0; i < 4; i++) {
        const coords = [];
        for (let j = 0; j < 4; j++) {
            if (dir === 'left') coords.push([i, j]);
            else if (dir === 'right') coords.push([i, 3 - j]);
            else if (dir === 'up') coords.push([j, i]);
            else coords.push([3 - j, i]);
        }
        lines.push(coords);
    }
    return lines;
}

function applyMove(board, dir) {
    const next = board.map(function (row) { return row.slice(); });
    let moved = false;
    let gained = 0;
    linesFor(dir).forEach(function (coords) {
        const values = coords.map(function (pair) { return next[pair[0]][pair[1]]; });
        const result = slide(values);
        gained += result.gained;
        result.line.forEach(function (v, i) {
            const r = coords[i][0];
            const c = coords[i][1];
            if (next[r][c] !== v) moved = true;
            next[r][c] = v;
        });
    });
    return { board: next, moved: moved, gained: gained };
}

function hasMove(board) {
    for (let r = 0; r < 4; r++) {
        for (let c = 0; c < 4; c++) {
            if (board[r][c] === 0) return true;
            if (c + 1 < 4 && board[r][c] === board[r][c + 1]) return true;
            if (r + 1 < 4 && board[r][c] === board[r + 1][c]) return true;
        }
    }
    return false;
}

function spawnTile(board) {
    const empty = [];
    for (let r = 0; r < 4; r++) {
        for (let c = 0; c < 4; c++) {
            if (board[r][c] === 0) empty.push([r, c]);
        }
    }
    if (!empty.length) return board;
    const spot = empty[Math.floor(Math.random() * empty.length)];
    const next = board.map(function (row) { return row.slice(); });
    next[spot[0]][spot[1]] = Math.random() < 0.9 ? 2 : 4;
    return next;
}

function tileColor(value) {
    const palette = {
        2: ['#eee4da', '#776e65'],
        4: ['#ede0c8', '#776e65'],
        8: ['#f2b179', '#f9f6f2'],
        16: ['#f59563', '#f9f6f2'],
        32: ['#f67c5f', '#f9f6f2'],
        64: ['#f65e3b', '#f9f6f2'],
        128: ['#edcf72', '#f9f6f2'],
        256: ['#edcc61', '#f9f6f2'],
        512: ['#edc850', '#f9f6f2'],
        1024: ['#edc53f', '#f9f6f2'],
        2048: ['#edc22e', '#f9f6f2']
    };
    if (!value) return ['transparent', 'transparent'];
    return palette[value] || ['#3c3a32', '#f9f6f2'];
}

function readBest() {
    try { return Number(localStorage.getItem('algods-2048-best')) || 0; }
    catch (e) { return 0; }
}

function writeBest(value) {
    try { localStorage.setItem('algods-2048-best', String(value)); }
    catch (e) {}
}

if (typeof document !== 'undefined') {
    boot2048();
}

function boot2048() {
    const boardEl = document.getElementById('board');
    const scoreEl = document.getElementById('score');
    const bestEl = document.getElementById('best');
    const statusEl = document.getElementById('status');
    const tiles = [];
    let board;
    let score;
    let best = readBest();
    let over = false;
    let won = false;

    for (let i = 0; i < 16; i++) {
        const cell = document.createElement('div');
        cell.className = 'cell';
        const tile = document.createElement('div');
        tile.className = 'tile';
        cell.appendChild(tile);
        boardEl.appendChild(cell);
        tiles.push(tile);
    }

    function paint() {
        for (let r = 0; r < 4; r++) {
            for (let c = 0; c < 4; c++) {
                const value = board[r][c];
                const el = tiles[r * 4 + c];
                const colors = tileColor(value);
                el.textContent = value ? String(value) : '';
                el.style.background = colors[0];
                el.style.color = colors[1];
                el.classList.toggle('small', value >= 1024);
            }
        }
        scoreEl.textContent = String(score);
        bestEl.textContent = String(best);
    }

    function setup() {
        board = spawnTile(spawnTile([
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0]
        ]));
        score = 0;
        over = false;
        won = false;
        statusEl.textContent = 'Arrow keys or swipe to slide.';
        paint();
    }

    function act(dir) {
        if (over) return;
        const result = applyMove(board, dir);
        if (!result.moved) return;
        board = spawnTile(result.board);
        score += result.gained;
        if (score > best) {
            best = score;
            writeBest(best);
        }
        if (!won && board.some(function (row) { return row.some(function (v) { return v >= 2048; }); })) {
            won = true;
            statusEl.textContent = 'You made 2048. Keep going, or start a new game.';
        }
        if (!hasMove(board)) {
            over = true;
            statusEl.textContent = won ? 'No moves left. You made 2048.' : 'No moves left.';
        }
        paint();
    }

    document.addEventListener('keydown', function (event) {
        if (event.metaKey || event.ctrlKey || event.altKey) return;
        const map = { ArrowLeft: 'left', ArrowRight: 'right', ArrowUp: 'up', ArrowDown: 'down' };
        if (!map[event.key]) return;
        event.preventDefault();
        act(map[event.key]);
    });

    document.querySelectorAll('[data-dir]').forEach(function (button) {
        button.addEventListener('click', function () { act(button.dataset.dir); });
    });

    let startX = 0;
    let startY = 0;
    boardEl.addEventListener('pointerdown', function (event) {
        startX = event.clientX;
        startY = event.clientY;
    });
    boardEl.addEventListener('pointerup', function (event) {
        const dx = event.clientX - startX;
        const dy = event.clientY - startY;
        if (Math.hypot(dx, dy) < 24) return;
        if (Math.abs(dx) > Math.abs(dy)) act(dx > 0 ? 'right' : 'left');
        else act(dy > 0 ? 'down' : 'up');
    });

    document.getElementById('restart').addEventListener('click', setup);
    setup();
}
