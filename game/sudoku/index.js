function shuffle(list) {
    const copy = list.slice();
    for (let i = copy.length - 1; i > 0; i--) {
        const j = Math.floor(Math.random() * (i + 1));
        const tmp = copy[i];
        copy[i] = copy[j];
        copy[j] = tmp;
    }
    return copy;
}

function filledBoard() {
    const base = [
        [1, 2, 3, 4, 5, 6, 7, 8, 9],
        [4, 5, 6, 7, 8, 9, 1, 2, 3],
        [7, 8, 9, 1, 2, 3, 4, 5, 6],
        [2, 3, 4, 5, 6, 7, 8, 9, 1],
        [5, 6, 7, 8, 9, 1, 2, 3, 4],
        [8, 9, 1, 2, 3, 4, 5, 6, 7],
        [3, 4, 5, 6, 7, 8, 9, 1, 2],
        [6, 7, 8, 9, 1, 2, 3, 4, 5],
        [9, 1, 2, 3, 4, 5, 6, 7, 8]
    ];
    const digits = shuffle([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let rows = base.map(function (row) {
        return row.map(function (n) { return digits[n - 1]; });
    });
    const banded = [];
    shuffle([0, 1, 2]).forEach(function (band) {
        shuffle([0, 1, 2]).forEach(function (offset) {
            banded.push(rows[band * 3 + offset]);
        });
    });
    const colOrder = [];
    shuffle([0, 1, 2]).forEach(function (stack) {
        shuffle([0, 1, 2]).forEach(function (offset) {
            colOrder.push(stack * 3 + offset);
        });
    });
    return banded.map(function (row) {
        return colOrder.map(function (c) { return row[c]; });
    });
}

function boxId(r, c) {
    return Math.floor(r / 3) * 3 + Math.floor(c / 3);
}

function countSolutions(board, limit) {
    const rows = Array.from({ length: 9 }, function () { return Array(10).fill(false); });
    const cols = Array.from({ length: 9 }, function () { return Array(10).fill(false); });
    const boxes = Array.from({ length: 9 }, function () { return Array(10).fill(false); });
    const empty = [];
    for (let r = 0; r < 9; r++) {
        for (let c = 0; c < 9; c++) {
            const value = board[r][c];
            if (!value) empty.push([r, c]);
            else {
                rows[r][value] = true;
                cols[c][value] = true;
                boxes[boxId(r, c)][value] = true;
            }
        }
    }
    let count = 0;
    let steps = 0;
    function dfs() {
        if (count >= limit || steps > 250000) return;
        steps++;
        let best = null;
        let options = null;
        for (let i = 0; i < empty.length; i++) {
            const r = empty[i][0];
            const c = empty[i][1];
            if (board[r][c]) continue;
            const b = boxId(r, c);
            const opts = [];
            for (let n = 1; n <= 9; n++) {
                if (!rows[r][n] && !cols[c][n] && !boxes[b][n]) opts.push(n);
            }
            if (opts.length === 0) return;
            if (!best || opts.length < options.length) {
                best = [r, c, b];
                options = opts;
                if (opts.length === 1) break;
            }
        }
        if (!best) {
            count++;
            return;
        }
        const r = best[0];
        const c = best[1];
        const b = best[2];
        for (let i = 0; i < options.length; i++) {
            const n = options[i];
            board[r][c] = n;
            rows[r][n] = cols[c][n] = boxes[b][n] = true;
            dfs();
            board[r][c] = 0;
            rows[r][n] = cols[c][n] = boxes[b][n] = false;
            if (count >= limit || steps > 250000) return;
        }
    }
    dfs();
    if (steps > 250000) return 2;
    return count;
}

function makePuzzle(solution) {
    const puzzle = solution.map(function (row) { return row.slice(); });
    const order = shuffle(Array.from({ length: 81 }, function (_, i) { return i; }));
    let removed = 0;
    for (let i = 0; i < order.length && removed < 40; i++) {
        const idx = order[i];
        const r = Math.floor(idx / 9);
        const c = idx % 9;
        const backup = puzzle[r][c];
        puzzle[r][c] = 0;
        const copy = puzzle.map(function (row) { return row.slice(); });
        if (countSolutions(copy, 2) !== 1) puzzle[r][c] = backup;
        else removed++;
    }
    return puzzle;
}

function isSolved(board) {
    function ok(values) {
        const seen = new Set(values);
        return seen.size === 9 && !seen.has(0);
    }
    for (let i = 0; i < 9; i++) {
        if (!ok(board[i])) return false;
        if (!ok(board.map(function (row) { return row[i]; }))) return false;
        const br = Math.floor(i / 3) * 3;
        const bc = (i % 3) * 3;
        const box = [];
        for (let r = 0; r < 3; r++) {
            for (let c = 0; c < 3; c++) box.push(board[br + r][bc + c]);
        }
        if (!ok(box)) return false;
    }
    return true;
}

function conflicts(board) {
    const bad = new Set();
    function mark(list) {
        const map = new Map();
        list.forEach(function (pair) {
            const value = board[pair[0]][pair[1]];
            if (!value) return;
            if (!map.has(value)) map.set(value, []);
            map.get(value).push(pair[0] + ',' + pair[1]);
        });
        map.forEach(function (cells) {
            if (cells.length > 1) cells.forEach(function (k) { bad.add(k); });
        });
    }
    for (let i = 0; i < 9; i++) {
        mark(Array.from({ length: 9 }, function (_, c) { return [i, c]; }));
        mark(Array.from({ length: 9 }, function (_, r) { return [r, i]; }));
        const br = Math.floor(i / 3) * 3;
        const bc = (i % 3) * 3;
        const box = [];
        for (let r = 0; r < 3; r++) {
            for (let c = 0; c < 3; c++) box.push([br + r, bc + c]);
        }
        mark(box);
    }
    return bad;
}

if (typeof document !== 'undefined') {
    bootSudoku();
}

function bootSudoku() {
    const gridEl = document.getElementById('grid');
    const padEl = document.getElementById('pad');
    const statusEl = document.getElementById('status');
    const buttons = [];
    let puzzle = [];
    let player = [];
    let selected = 0;
    let over = false;

    for (let i = 0; i < 81; i++) {
        const button = document.createElement('button');
        button.type = 'button';
        button.className = 'cell';
        button.addEventListener('click', function () {
            selected = i;
            render();
        });
        gridEl.appendChild(button);
        buttons.push(button);
    }

    for (let n = 1; n <= 9; n++) {
        const button = document.createElement('button');
        button.type = 'button';
        button.textContent = String(n);
        button.addEventListener('click', function () { enter(n); });
        padEl.appendChild(button);
    }
    const erase = document.createElement('button');
    erase.type = 'button';
    erase.className = 'wide';
    erase.textContent = 'Erase';
    erase.addEventListener('click', function () { enter(0); });
    padEl.appendChild(erase);

    function render() {
        const bad = player.length ? conflicts(player) : new Set();
        const sr = Math.floor(selected / 9);
        const sc = selected % 9;
        const selectedValue = player.length ? player[sr][sc] : 0;
        for (let i = 0; i < 81; i++) {
            const r = Math.floor(i / 9);
            const c = i % 9;
            const value = player.length ? player[r][c] : 0;
            const classes = ['cell'];
            const given = puzzle.length && puzzle[r][c] !== 0;
            if (given) classes.push('given');
            if (i === selected) classes.push('selected');
            else if (player.length && (r === sr || c === sc || (Math.floor(r / 3) === Math.floor(sr / 3) && Math.floor(c / 3) === Math.floor(sc / 3)))) {
                classes.push('peer');
            }
            if (selectedValue && value === selectedValue) classes.push('same');
            if (bad.has(r + ',' + c)) classes.push('conflict');
            buttons[i].className = classes.join(' ');
            buttons[i].textContent = value ? String(value) : '';
        }
    }

    function enter(n) {
        if (!player.length || over) return;
        const r = Math.floor(selected / 9);
        const c = selected % 9;
        if (puzzle[r][c] !== 0) return;
        player[r][c] = n;
        if (isSolved(player)) {
            over = true;
            statusEl.textContent = 'Solved.';
        }
        render();
    }

    function newPuzzle() {
        over = false;
        statusEl.textContent = 'Building a puzzle…';
        setTimeout(function () {
            puzzle = makePuzzle(filledBoard());
            player = puzzle.map(function (row) { return row.slice(); });
            selected = 0;
            statusEl.textContent = 'Each row, column, and box needs the digits 1–9 once.';
            render();
        }, 20);
    }

    document.addEventListener('keydown', function (event) {
        if (event.metaKey || event.ctrlKey || event.altKey) return;
        if (!player.length) return;
        if (event.key >= '1' && event.key <= '9') {
            enter(Number(event.key));
            return;
        }
        if (event.key === 'Backspace' || event.key === 'Delete' || event.key === '0') {
            event.preventDefault();
            enter(0);
            return;
        }
        const r = Math.floor(selected / 9);
        const c = selected % 9;
        if (event.key === 'ArrowLeft' && c > 0) selected -= 1;
        else if (event.key === 'ArrowRight' && c < 8) selected += 1;
        else if (event.key === 'ArrowUp' && r > 0) selected -= 9;
        else if (event.key === 'ArrowDown' && r < 8) selected += 9;
        else return;
        event.preventDefault();
        render();
    });

    document.getElementById('restart').addEventListener('click', newPuzzle);
    newPuzzle();
}
