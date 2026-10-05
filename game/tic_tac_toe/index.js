const LINES = [
    [0, 1, 2], [3, 4, 5], [6, 7, 8],
    [0, 3, 6], [1, 4, 7], [2, 5, 8],
    [0, 4, 8], [2, 4, 6]
];

function outcome(board) {
    for (let i = 0; i < LINES.length; i++) {
        const a = LINES[i][0];
        const b = LINES[i][1];
        const c = LINES[i][2];
        if (board[a] && board[a] === board[b] && board[a] === board[c]) return board[a];
    }
    if (board.every(function (cell) { return cell; })) return 'draw';
    return null;
}

function winningLine(board) {
    for (let i = 0; i < LINES.length; i++) {
        const line = LINES[i];
        if (board[line[0]] && board[line[0]] === board[line[1]] && board[line[0]] === board[line[2]]) {
            return line;
        }
    }
    return null;
}

function chooseMove(board) {
    const snapshot = board.slice();
    const empty = [];
    snapshot.forEach(function (cell, index) {
        if (!cell) empty.push(index);
    });

    function find(player) {
        for (let i = 0; i < empty.length; i++) {
            const index = empty[i];
            snapshot[index] = player;
            const result = outcome(snapshot);
            snapshot[index] = '';
            if (result === player) return index;
        }
        return null;
    }

    const win = find('X');
    if (win !== null) return win;
    const block = find('O');
    if (block !== null) return block;
    if (!snapshot[4]) return 4;
    const corners = [0, 2, 6, 8].filter(function (index) { return !snapshot[index]; });
    if (corners.length) return corners[Math.floor(Math.random() * corners.length)];
    return empty[Math.floor(Math.random() * empty.length)];
}

if (typeof document !== 'undefined') {
    bootTicTacToe();
}

function bootTicTacToe() {
    const boardEl = document.getElementById('board');
    const statusEl = document.getElementById('status');
    const buttons = [];
    let board = Array(9).fill('');
    let over = false;
    let locked = false;
    let round = 0;

    for (let i = 0; i < 9; i++) {
        const button = document.createElement('button');
        button.type = 'button';
        button.className = 'cell';
        button.addEventListener('click', function () { play(i); });
        boardEl.appendChild(button);
        buttons.push(button);
    }

    function render() {
        const line = winningLine(board);
        buttons.forEach(function (button, index) {
            const mark = board[index];
            button.textContent = mark;
            button.className = 'cell';
            if (mark === 'X') button.classList.add('x');
            if (mark === 'O') button.classList.add('o');
            if (line && line.indexOf(index) !== -1) button.classList.add('win');
        });
    }

    function finish(result) {
        over = true;
        locked = false;
        if (result === 'draw') statusEl.textContent = 'Draw.';
        else if (result === 'O') statusEl.textContent = 'You win.';
        else statusEl.textContent = 'Computer wins.';
        render();
    }

    function play(index) {
        if (locked || over || board[index]) return;
        board[index] = 'O';
        const result = outcome(board);
        if (result) {
            finish(result);
            return;
        }
        locked = true;
        statusEl.textContent = 'Computer is thinking…';
        render();
        const ticket = round;
        setTimeout(function () {
            if (ticket !== round) return;
            const move = chooseMove(board);
            if (move != null) board[move] = 'X';
            const after = outcome(board);
            locked = false;
            if (after) finish(after);
            else {
                statusEl.textContent = 'Your turn. You are O.';
                render();
            }
        }, 180);
    }

    function newGame() {
        round++;
        board = Array(9).fill('');
        over = false;
        locked = false;
        statusEl.textContent = 'Your turn. You are O.';
        render();
    }

    document.getElementById('restart').addEventListener('click', newGame);
    newGame();
}
