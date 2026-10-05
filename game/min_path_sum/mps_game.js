function randomGrid(n) {
    return Array.from({ length: n }, function () {
        return Array.from({ length: n }, function () {
            return 1 + Math.floor(Math.random() * 9);
        });
    });
}

function bestPath(grid) {
    var h = grid.length;
    var w = grid[0].length;
    var dp = Array.from({ length: h }, function () { return Array(w).fill(0); });
    dp[0][0] = grid[0][0];
    for (var c = 1; c < w; c++) dp[0][c] = dp[0][c - 1] + grid[0][c];
    for (var r = 1; r < h; r++) dp[r][0] = dp[r - 1][0] + grid[r][0];
    for (var y = 1; y < h; y++) {
        for (var x = 1; x < w; x++) {
            dp[y][x] = grid[y][x] + Math.min(dp[y - 1][x], dp[y][x - 1]);
        }
    }
    var path = [];
    var row = h - 1;
    var col = w - 1;
    path.push([row, col]);
    while (row > 0 || col > 0) {
        if (row === 0) col--;
        else if (col === 0) row--;
        else if (dp[row - 1][col] <= dp[row][col - 1]) row--;
        else col--;
        path.push([row, col]);
    }
    path.reverse();
    return { sum: dp[h - 1][w - 1], path: path };
}

if (typeof document !== 'undefined') {
    bootPath();
}

function bootPath() {
    var N = 4;
    var gridEl = document.getElementById('grid');
    var sumEl = document.getElementById('sum');
    var statusEl = document.getElementById('status');
    var rightBtn = document.getElementById('right');
    var downBtn = document.getElementById('down');
    var undoBtn = document.getElementById('undo');
    var bestBtn = document.getElementById('best');
    var cells = [];
    var grid, path, optimal, showBest;

    function pathSum() {
        return path.reduce(function (sum, pair) {
            return sum + grid[pair[0]][pair[1]];
        }, 0);
    }

    function atEnd() {
        var last = path[path.length - 1];
        return last[0] === N - 1 && last[1] === N - 1;
    }

    function render() {
        var here = path[path.length - 1];
        var onPath = {};
        path.forEach(function (pair) { onPath[pair[0] + ',' + pair[1]] = true; });
        var gold = {};
        if (showBest) {
            optimal.path.forEach(function (pair) { gold[pair[0] + ',' + pair[1]] = true; });
        }
        cells.forEach(function (el, index) {
            var r = Math.floor(index / N);
            var c = index % N;
            var key = r + ',' + c;
            el.className = 'cell';
            if (onPath[key]) el.classList.add('path');
            if (r === here[0] && c === here[1]) el.classList.add('current');
            if (gold[key] && !onPath[key]) el.classList.add('best');
        });
        var sum = pathSum();
        sumEl.textContent = String(sum);
        var finished = atEnd();
        rightBtn.disabled = finished || here[1] === N - 1;
        downBtn.disabled = finished || here[0] === N - 1;
        undoBtn.disabled = path.length <= 1;
        bestBtn.disabled = !finished;
        if (!finished) {
            statusEl.textContent = 'Reach the bottom-right corner. Only right and down are allowed.';
        } else if (sum === optimal.sum) {
            statusEl.textContent = 'That is the best path. Sum ' + sum + '.';
        } else {
            statusEl.textContent = 'Your sum is ' + sum + '. The best sum is ' + optimal.sum + '.';
        }
    }

    function move(dr, dc) {
        if (atEnd()) return;
        var here = path[path.length - 1];
        var r = here[0] + dr;
        var c = here[1] + dc;
        if (r < 0 || c < 0 || r >= N || c >= N) return;
        path.push([r, c]);
        showBest = false;
        render();
    }

    function undo() {
        if (path.length <= 1) return;
        path.pop();
        showBest = false;
        render();
    }

    function setup() {
        grid = randomGrid(N);
        path = [[0, 0]];
        optimal = bestPath(grid);
        showBest = false;
        gridEl.innerHTML = '';
        cells = [];
        for (var r = 0; r < N; r++) {
            for (var c = 0; c < N; c++) {
                var el = document.createElement('div');
                el.className = 'cell';
                el.textContent = String(grid[r][c]);
                gridEl.appendChild(el);
                cells.push(el);
            }
        }
        render();
    }

    rightBtn.addEventListener('click', function () { move(0, 1); });
    downBtn.addEventListener('click', function () { move(1, 0); });
    undoBtn.addEventListener('click', undo);
    bestBtn.addEventListener('click', function () {
        if (!atEnd()) return;
        showBest = true;
        render();
    });
    document.getElementById('restart').addEventListener('click', setup);
    document.addEventListener('keydown', function (event) {
        if (event.metaKey || event.ctrlKey || event.altKey) return;
        if (event.key === 'ArrowRight') move(0, 1);
        else if (event.key === 'ArrowDown') move(1, 0);
        else if (event.key === 'ArrowLeft' || event.key === 'ArrowUp') undo();
        else return;
        event.preventDefault();
    });
    setup();
}
