const HEX_RADIUS = 2;
const HEX_DIRS = ['E', 'W', 'NE', 'NW', 'SE', 'SW'];

function hexKey(q, r) {
    return q + ',' + r;
}

function hexCells(radius) {
    const cells = [];
    for (let q = -radius; q <= radius; q++) {
        for (let r = -radius; r <= radius; r++) {
            if (Math.abs(q + r) <= radius) cells.push({ q: q, r: r });
        }
    }
    return cells;
}

function slideLine(values) {
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

function hexGroup(cell, dir) {
    if (dir === 'E' || dir === 'W') return 'r' + cell.r;
    if (dir === 'SE' || dir === 'NW') return 'q' + cell.q;
    return 's' + (cell.q + cell.r);
}

function sortHexLine(cells, dir) {
    const copy = cells.slice();
    copy.sort(function (a, b) {
        if (dir === 'E' || dir === 'NE') return b.q - a.q;
        if (dir === 'W' || dir === 'SW') return a.q - b.q;
        if (dir === 'SE') return b.r - a.r;
        return a.r - b.r;
    });
    return copy;
}

function emptyHex() {
    const board = {};
    hexCells(HEX_RADIUS).forEach(function (cell) {
        board[hexKey(cell.q, cell.r)] = 0;
    });
    return board;
}

function moveHex(board, dir) {
    const cells = hexCells(HEX_RADIUS);
    const groups = new Map();
    cells.forEach(function (cell) {
        const name = hexGroup(cell, dir);
        if (!groups.has(name)) groups.set(name, []);
        groups.get(name).push(cell);
    });
    const next = emptyHex();
    let moved = false;
    let gained = 0;
    groups.forEach(function (group) {
        const ordered = sortHexLine(group, dir);
        const values = ordered.map(function (cell) { return board[hexKey(cell.q, cell.r)] || 0; });
        const result = slideLine(values);
        gained += result.gained;
        result.line.forEach(function (value, index) {
            const k = hexKey(ordered[index].q, ordered[index].r);
            if ((board[k] || 0) !== value) moved = true;
            next[k] = value;
        });
    });
    return { board: next, moved: moved, gained: gained };
}

function hasHexMove(board) {
    return HEX_DIRS.some(function (dir) { return moveHex(board, dir).moved; });
}

function spawnHex(board) {
    const empty = Object.keys(board).filter(function (k) { return board[k] === 0; });
    if (!empty.length) return board;
    const next = Object.assign({}, board);
    next[empty[Math.floor(Math.random() * empty.length)]] = Math.random() < 0.9 ? 2 : 4;
    return next;
}

function hexColor(value) {
    const palette = {
        2: '#eee4da',
        4: '#ede0c8',
        8: '#f2b179',
        16: '#f59563',
        32: '#f67c5f',
        64: '#f65e3b',
        128: '#edcf72',
        256: '#edcc61',
        512: '#edc850',
        1024: '#edc53f',
        2048: '#edc22e'
    };
    return palette[value] || '#3c3a32';
}

function dirFromSwipe(dx, dy) {
    const deg = Math.atan2(dy, dx) * 180 / Math.PI;
    const dirs = [['E', 0], ['SE', 60], ['SW', 120], ['W', 180], ['NW', -120], ['NE', -60]];
    let best = 'E';
    let bestDiff = 999;
    dirs.forEach(function (pair) {
        let diff = Math.abs(deg - pair[1]);
        if (diff > 180) diff = 360 - diff;
        if (diff < bestDiff) {
            bestDiff = diff;
            best = pair[0];
        }
    });
    return best;
}

if (typeof document !== 'undefined') {
    bootHex();
}

function bootHex() {
    const canvas = document.getElementById('board');
    const ctx = canvas.getContext('2d');
    const scoreEl = document.getElementById('score');
    const bestEl = document.getElementById('best');
    const statusEl = document.getElementById('status');
    let board = emptyHex();
    let score = 0;
    let best = 0;
    let over = false;
    let won = false;

    try { best = Number(localStorage.getItem('algods-hex2048-best')) || 0; }
    catch (e) { best = 0; }

    function pixel(q, r) {
        const size = 48;
        return {
            x: size * (Math.sqrt(3) * q + (Math.sqrt(3) / 2) * r) + 260,
            y: size * 1.5 * r + 230
        };
    }

    function draw() {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        hexCells(HEX_RADIUS).forEach(function (cell) {
            const point = pixel(cell.q, cell.r);
            const value = board[hexKey(cell.q, cell.r)];
            ctx.beginPath();
            for (let i = 0; i < 6; i++) {
                const angle = (Math.PI / 3) * i - Math.PI / 2;
                const x = point.x + 42 * Math.cos(angle);
                const y = point.y + 42 * Math.sin(angle);
                if (i === 0) ctx.moveTo(x, y);
                else ctx.lineTo(x, y);
            }
            ctx.closePath();
            ctx.fillStyle = value ? hexColor(value) : '#2c2433';
            ctx.fill();
            ctx.lineWidth = 4;
            ctx.strokeStyle = '#1b1522';
            ctx.stroke();
            if (value) {
                ctx.fillStyle = value <= 4 ? '#776e65' : '#f9f6f2';
                ctx.font = (value >= 1024 ? '800 14px ' : '800 18px ') + 'Outfit, ui-sans-serif, system-ui, sans-serif';
                ctx.textAlign = 'center';
                ctx.textBaseline = 'middle';
                ctx.fillText(String(value), point.x, point.y);
            }
        });
        scoreEl.textContent = String(score);
        bestEl.textContent = String(best);
    }

    function setup() {
        board = spawnHex(spawnHex(emptyHex()));
        score = 0;
        over = false;
        won = false;
        statusEl.textContent = 'Slide in any of the six directions.';
        draw();
    }

    function act(dir) {
        if (over) return;
        const result = moveHex(board, dir);
        if (!result.moved) return;
        board = spawnHex(result.board);
        score += result.gained;
        if (score > best) {
            best = score;
            try { localStorage.setItem('algods-hex2048-best', String(best)); }
            catch (e) {}
        }
        const reached = Object.keys(board).some(function (k) { return board[k] >= 2048; });
        if (!won && reached) {
            won = true;
            statusEl.textContent = 'You made 2048. Keep going, or start a new game.';
        }
        if (!hasHexMove(board)) {
            over = true;
            statusEl.textContent = won ? 'No moves left. You made 2048.' : 'No moves left.';
        }
        draw();
    }

    document.addEventListener('keydown', function (event) {
        if (event.metaKey || event.ctrlKey || event.altKey) return;
        const map = {
            KeyA: 'W', ArrowLeft: 'W',
            KeyD: 'E', ArrowRight: 'E',
            KeyQ: 'NW',
            KeyW: 'NE', ArrowUp: 'NE',
            KeyZ: 'SW',
            KeyX: 'SE', ArrowDown: 'SE'
        };
        if (!map[event.code] && !map[event.key]) return;
        event.preventDefault();
        act(map[event.code] || map[event.key]);
    });

    document.querySelectorAll('[data-dir]').forEach(function (button) {
        button.addEventListener('click', function () { act(button.dataset.dir); });
    });

    let startX = 0;
    let startY = 0;
    canvas.addEventListener('pointerdown', function (event) {
        startX = event.clientX;
        startY = event.clientY;
    });
    canvas.addEventListener('pointerup', function (event) {
        const dx = event.clientX - startX;
        const dy = event.clientY - startY;
        if (Math.hypot(dx, dy) < 24) return;
        act(dirFromSwipe(dx, dy));
    });

    document.getElementById('restart').addEventListener('click', setup);
    setup();
}
