(function (factory) {
    var Cube = factory();
    if (typeof module === 'object' && module.exports) module.exports = Cube;
    if (typeof document !== 'undefined') startCube(Cube);
})(function () {
    // Clockwise quarter turns, as viewed looking at that face.
    // dir is the number of +90° right-hand steps around the axis (negative wraps).
    var FACE = {
        U: { axis: 'y', layer: 1, dir: -1 },
        D: { axis: 'y', layer: -1, dir: 1 },
        R: { axis: 'x', layer: 1, dir: -1 },
        L: { axis: 'x', layer: -1, dir: 1 },
        F: { axis: 'z', layer: 1, dir: -1 },
        B: { axis: 'z', layer: -1, dir: 1 },
        M: { axis: 'x', layer: 0, dir: 1 },
        E: { axis: 'y', layer: 0, dir: 1 },
        S: { axis: 'z', layer: 0, dir: -1 }
    };

    var TURN_POS = {
        x: function (p) { return { x: p.x, y: -p.z, z: p.y }; },
        y: function (p) { return { x: p.z, y: p.y, z: -p.x }; },
        z: function (p) { return { x: -p.y, y: p.x, z: p.z }; }
    };

    var TURN_COLOR = {
        x: function (c) {
            return { px: c.px, nx: c.nx, py: c.nz, ny: c.pz, pz: c.py, nz: c.ny };
        },
        y: function (c) {
            return { px: c.pz, nx: c.nz, py: c.py, ny: c.ny, pz: c.nx, nz: c.px };
        },
        z: function (c) {
            return { px: c.ny, nx: c.py, py: c.px, ny: c.nx, pz: c.pz, nz: c.nz };
        }
    };

    function createSolved() {
        var cubies = [];
        var x, y, z;
        for (x = -1; x <= 1; x++) {
            for (y = -1; y <= 1; y++) {
                for (z = -1; z <= 1; z++) {
                    if (x === 0 && y === 0 && z === 0) continue;
                    cubies.push({
                        x: x,
                        y: y,
                        z: z,
                        color: {
                            px: x === 1 ? 'R' : null,
                            nx: x === -1 ? 'O' : null,
                            py: y === 1 ? 'W' : null,
                            ny: y === -1 ? 'Y' : null,
                            pz: z === 1 ? 'G' : null,
                            nz: z === -1 ? 'B' : null
                        }
                    });
                }
            }
        }
        return cubies;
    }

    function apply(cubies, move) {
        var steps = ((move.dir % 4) + 4) % 4;
        var picked = cubies.filter(function (c) { return c[move.axis] === move.layer; });
        var s, i, c, p;
        for (s = 0; s < steps; s++) {
            for (i = 0; i < picked.length; i++) {
                c = picked[i];
                p = TURN_POS[move.axis](c);
                c.x = p.x;
                c.y = p.y;
                c.z = p.z;
                c.color = TURN_COLOR[move.axis](c.color);
            }
        }
        return cubies;
    }

    function parse(seq) {
        var moves = [];
        var re = /([UDLRFBMES])(2|'|′|’)?/g;
        var text = String(seq).toUpperCase();
        var match;
        while ((match = re.exec(text))) {
            var base = FACE[match[1]];
            var mod = match[2] || '';
            var dir = base.dir;
            if (mod === '2') dir = 2;
            else if (mod) dir = -base.dir;
            moves.push({ axis: base.axis, layer: base.layer, dir: dir });
        }
        return moves;
    }

    function applySeq(cubies, seq) {
        parse(seq).forEach(function (move) { apply(cubies, move); });
        return cubies;
    }

    function isSolved(cubies) {
        return cubies.every(function (c) {
            if (c.x === 1 && c.color.px !== 'R') return false;
            if (c.x === -1 && c.color.nx !== 'O') return false;
            if (c.y === 1 && c.color.py !== 'W') return false;
            if (c.y === -1 && c.color.ny !== 'Y') return false;
            if (c.z === 1 && c.color.pz !== 'G') return false;
            if (c.z === -1 && c.color.nz !== 'B') return false;
            return true;
        });
    }

    function moveName(move) {
        var letter = '';
        Object.keys(FACE).forEach(function (key) {
            if (FACE[key].axis === move.axis && FACE[key].layer === move.layer) letter = key;
        });
        var steps = ((move.dir % 4) + 4) % 4;
        if (steps === 0 || !letter) return '';
        if (steps === 2) return letter + '2';
        var natural = ((FACE[letter].dir % 4) + 4) % 4;
        return letter + (steps === natural ? '' : "'");
    }

    function invert(move) {
        return {
            axis: move.axis,
            layer: move.layer,
            dir: move.dir === 2 ? 2 : -move.dir
        };
    }

    function scrambleMoves(count) {
        var letters = ['U', 'D', 'L', 'R', 'F', 'B'];
        var moves = [];
        var lastAxis = '';
        while (moves.length < count) {
            var letter = letters[Math.floor(Math.random() * letters.length)];
            var base = FACE[letter];
            if (base.axis === lastAxis) continue;
            var roll = Math.floor(Math.random() * 3);
            var dir = roll === 1 ? -base.dir : roll === 2 ? 2 : base.dir;
            moves.push({ axis: base.axis, layer: base.layer, dir: dir });
            lastAxis = base.axis;
        }
        return moves;
    }

    function dragMove(normal, drag, cubie) {
        var dot = normal.x * drag.x + normal.y * drag.y + normal.z * drag.z;
        var d = {
            x: drag.x - normal.x * dot,
            y: drag.y - normal.y * dot,
            z: drag.z - normal.z * dot
        };
        if (Math.hypot(d.x, d.y, d.z) < 1e-8) return null;
        var a = {
            x: normal.y * d.z - normal.z * d.y,
            y: normal.z * d.x - normal.x * d.z,
            z: normal.x * d.y - normal.y * d.x
        };
        var axis = 'x';
        var best = Math.abs(a.x);
        if (Math.abs(a.y) > best) { axis = 'y'; best = Math.abs(a.y); }
        if (Math.abs(a.z) > best) axis = 'z';
        return { axis: axis, layer: cubie[axis], dir: a[axis] > 0 ? 1 : -1 };
    }

    function screenToWorld(dx, dy, yawDeg, pitchDeg) {
        var yaw = yawDeg * Math.PI / 180;
        var pitch = pitchDeg * Math.PI / 180;
        var x1 = dx;
        var y1 = dy * Math.cos(pitch);
        var z1 = -dy * Math.sin(pitch);
        var cy = Math.cos(yaw);
        var sy = Math.sin(yaw);
        return {
            x: x1 * cy - z1 * sy,
            y: -y1,
            z: x1 * sy + z1 * cy
        };
    }

    function findCubie(cubies, x, y, z) {
        for (var i = 0; i < cubies.length; i++) {
            if (cubies[i].x === x && cubies[i].y === y && cubies[i].z === z) return cubies[i];
        }
        return null;
    }

    function inspect(cubies) {
        function at(x, y, z, face) {
            var cubie = findCubie(cubies, x, y, z);
            return cubie && cubie.color[face] ? cubie.color[face] : '.';
        }
        function rows(loop) {
            var lines = [];
            loop(function (cells) { lines.push(cells.join(' ')); });
            return lines.join('\n');
        }
        var faces = {
            U: rows(function (add) {
                var z, x, cells;
                for (z = -1; z <= 1; z++) {
                    cells = [];
                    for (x = -1; x <= 1; x++) cells.push(at(x, 1, z, 'py'));
                    add(cells);
                }
            }),
            D: rows(function (add) {
                var z, x, cells;
                for (z = 1; z >= -1; z--) {
                    cells = [];
                    for (x = -1; x <= 1; x++) cells.push(at(x, -1, z, 'ny'));
                    add(cells);
                }
            }),
            F: rows(function (add) {
                var y, x, cells;
                for (y = 1; y >= -1; y--) {
                    cells = [];
                    for (x = -1; x <= 1; x++) cells.push(at(x, y, 1, 'pz'));
                    add(cells);
                }
            }),
            B: rows(function (add) {
                var y, x, cells;
                for (y = 1; y >= -1; y--) {
                    cells = [];
                    for (x = 1; x >= -1; x--) cells.push(at(x, y, -1, 'nz'));
                    add(cells);
                }
            }),
            R: rows(function (add) {
                var y, z, cells;
                for (y = 1; y >= -1; y--) {
                    cells = [];
                    for (z = 1; z >= -1; z--) cells.push(at(1, y, z, 'px'));
                    add(cells);
                }
            }),
            L: rows(function (add) {
                var y, z, cells;
                for (y = 1; y >= -1; y--) {
                    cells = [];
                    for (z = -1; z <= 1; z++) cells.push(at(-1, y, z, 'nx'));
                    add(cells);
                }
            })
        };
        return ['U', 'D', 'F', 'B', 'R', 'L'].map(function (name) {
            return name + '\n' + faces[name];
        }).join('\n');
    }

    return {
        FACE: FACE,
        createSolved: createSolved,
        apply: apply,
        applySeq: applySeq,
        parse: parse,
        isSolved: isSolved,
        moveName: moveName,
        invert: invert,
        scrambleMoves: scrambleMoves,
        dragMove: dragMove,
        screenToWorld: screenToWorld,
        findCubie: findCubie,
        inspect: inspect
    };
});

var FACE_NORMAL = {
    px: { x: 1, y: 0, z: 0 },
    nx: { x: -1, y: 0, z: 0 },
    py: { x: 0, y: 1, z: 0 },
    ny: { x: 0, y: -1, z: 0 },
    pz: { x: 0, y: 0, z: 1 },
    nz: { x: 0, y: 0, z: -1 }
};

function startCube(Cube) {
    var viewport = document.getElementById('viewport');
    var rig = document.getElementById('rig');
    var cubeEl = document.getElementById('cube');
    var statusEl = document.getElementById('status');
    var captionEl = document.getElementById('caption');
    var turnsEl = document.getElementById('turns');
    var timeEl = document.getElementById('time');
    var bestEl = document.getElementById('best');
    var undoBtn = document.getElementById('undo');
    if (!viewport || !cubeEl) return;

    var CUBIE = 52;
    var STEP = 58;
    var FACE_NAMES = ['px', 'nx', 'py', 'ny', 'pz', 'nz'];
    viewport.style.setProperty('--cubie', CUBIE + 'px');
    viewport.style.setProperty('--half', (CUBIE / 2) + 'px');

    var state = Cube.createSolved();
    var queue = [];
    var history = [];
    var log = [];
    var busy = false;
    var epoch = 0;
    var scrambling = false;
    var fromScramble = false;
    var timing = false;
    var startedAt = 0;
    var elapsed = 0;
    var timerId = 0;
    var yaw = -36;
    var pitch = -28;
    var idle = true;
    var drag = null;
    var turnMs = window.matchMedia('(prefers-reduced-motion: reduce)').matches ? 0 : 200;

    function formatTime(ms) {
        var total = Math.max(0, Math.round(ms));
        var m = Math.floor(total / 60000);
        var s = Math.floor(total / 1000) % 60;
        var t = Math.floor(total / 100) % 10;
        return m + ':' + (s < 10 ? '0' : '') + s + '.' + t;
    }

    function readBest() {
        try {
            var n = Number(localStorage.getItem('cube-best-ms'));
            return n > 0 ? n : 0;
        } catch (err) {
            return 0;
        }
    }

    function saveBest(ms) {
        var rounded = Math.round(ms);
        var prev = readBest();
        if (!prev || rounded < prev) {
            try { localStorage.setItem('cube-best-ms', String(rounded)); } catch (err) { /* ignore */ }
        }
    }

    function renderTime() {
        var ms = timing ? performance.now() - startedAt : elapsed;
        timeEl.textContent = formatTime(ms);
    }

    function startTimer() {
        if (timing) return;
        timing = true;
        startedAt = performance.now() - elapsed;
        clearInterval(timerId);
        timerId = setInterval(renderTime, 100);
        renderTime();
    }

    function stopTimer() {
        if (timing) elapsed = performance.now() - startedAt;
        timing = false;
        clearInterval(timerId);
        renderTime();
    }

    function clearTimer() {
        timing = false;
        clearInterval(timerId);
        elapsed = 0;
        renderTime();
    }

    function renderHud() {
        turnsEl.textContent = String(Math.max(0, history.length));
        undoBtn.disabled = !history.length || scrambling;
        var best = readBest();
        bestEl.textContent = best ? formatTime(best) : '—';
        captionEl.textContent = log.length
            ? log.join(' ')
            : 'Drag a sticker to turn. Keys U D L R F B — shift reverses.';
        renderTime();
    }

    function renderRig() {
        pitch = Math.max(-89, Math.min(89, pitch));
        rig.style.transform = 'rotateX(' + pitch + 'deg) rotateY(' + yaw + 'deg)';
    }

    function place(cubie) {
        cubie.el.style.transform = 'translate3d(' + (cubie.x * STEP) + 'px,' + (-cubie.y * STEP) + 'px,' + (cubie.z * STEP) + 'px)';
    }

    function paint(cubie) {
        FACE_NAMES.forEach(function (face) {
            var el = cubie.el.querySelector('.face-' + face);
            var color = cubie.color[face];
            var sticker = el.querySelector('.sticker');
            if (!color) {
                if (sticker) sticker.remove();
                el.style.pointerEvents = 'none';
                return;
            }
            if (!sticker) {
                sticker = document.createElement('div');
                el.appendChild(sticker);
            }
            sticker.className = 'sticker s-' + color;
            el.style.pointerEvents = 'auto';
        });
    }

    function buildCubie(cubie) {
        var el = document.createElement('div');
        el.className = 'cubie';
        FACE_NAMES.forEach(function (face) {
            var faceEl = document.createElement('div');
            faceEl.className = 'face face-' + face;
            faceEl.dataset.face = face;
            faceEl._cubie = cubie;
            el.appendChild(faceEl);
        });
        cubie.el = el;
        return el;
    }

    function mount() {
        cubeEl.innerHTML = '';
        state.forEach(function (cubie) {
            buildCubie(cubie);
            place(cubie);
            paint(cubie);
            cubeEl.appendChild(cubie.el);
        });
    }

    function cssRotate(axis, dir) {
        var steps = ((dir % 4) + 4) % 4;
        var angle = steps * 90;
        if (axis === 'x' || axis === 'z') angle = -angle;
        var fn = axis === 'x' ? 'rotateX' : axis === 'y' ? 'rotateY' : 'rotateZ';
        return fn + '(' + angle + 'deg)';
    }

    function speak(text) {
        statusEl.textContent = text;
    }

    function afterCommit(recorded) {
        var solved = Cube.isSolved(state);
        if (solved) {
            stopTimer();
            if (fromScramble && recorded && history.length > 0) {
                saveBest(elapsed);
                speak('Solved in ' + formatTime(elapsed) + '.');
            } else if (!scrambling) {
                speak('I am the cube.');
            }
            if (recorded || history.length === 0) fromScramble = false;
        } else if (recorded) {
            if (!timing) startTimer();
            speak(log[log.length - 1] || 'I am the cube.');
        }
        renderHud();
    }

    function finishJob(job, token) {
        Cube.apply(state, job.move);
        if (job.record) {
            history.push(job.move);
            log.push(Cube.moveName(job.move));
        }
        mount();
        busy = false;
        if (token !== epoch) return;
        afterCommit(!!job.record);
        pump();
    }

    function startAnimation(job) {
        busy = true;
        var token = epoch;
        var ms = job.ms;
        var picked = state.filter(function (c) { return c[job.move.axis] === job.move.layer; });
        if (ms <= 0) {
            finishJob(job, token);
            return;
        }
        var turn = document.createElement('div');
        turn.className = 'turn';
        picked.forEach(function (cubie) { turn.appendChild(cubie.el); });
        cubeEl.appendChild(turn);
        var settled = false;
        function finish() {
            if (settled || token !== epoch) return;
            settled = true;
            if (turn.parentNode) turn.remove();
            finishJob(job, token);
        }
        window.setTimeout(finish, ms + 40);
        turn.addEventListener('transitionend', function (event) {
            if (event.target === turn && event.propertyName === 'transform') finish();
        });
        window.requestAnimationFrame(function () {
            if (token !== epoch) return;
            void turn.offsetWidth;
            turn.style.transition = 'transform ' + ms + 'ms cubic-bezier(.2,.7,.2,1)';
            turn.style.transform = cssRotate(job.move.axis, job.move.dir);
        });
    }

    function pump() {
        if (busy) return;
        if (!queue.length) {
            if (scrambling) {
                scrambling = false;
                fromScramble = true;
                clearTimer();
                speak('Your turn.');
                renderHud();
            }
            return;
        }
        var job = queue.shift();
        if (job.undo) {
            var last = history.pop();
            log.pop();
            if (!last) {
                pump();
                return;
            }
            startAnimation({ move: Cube.invert(last), record: false, ms: turnMs });
            return;
        }
        startAnimation(job);
    }

    function enqueue(move, record, ms) {
        queue.push({ move: move, record: !!record, ms: ms == null ? turnMs : ms });
        pump();
    }

    function userMove(move) {
        if (scrambling || !move) return;
        idle = false;
        enqueue(move, true, turnMs);
    }

    function userSeq(seq) {
        Cube.parse(seq).forEach(function (move) { userMove(move); });
    }

    function resetCube() {
        epoch += 1;
        queue = [];
        busy = false;
        scrambling = false;
        fromScramble = false;
        history = [];
        log = [];
        state = Cube.createSolved();
        clearTimer();
        mount();
        speak('I am the cube.');
        renderHud();
    }

    function scramble() {
        if (scrambling) return;
        idle = false;
        epoch += 1;
        queue = [];
        busy = false;
        scrambling = true;
        fromScramble = false;
        history = [];
        log = [];
        clearTimer();
        mount();
        speak('Scrambling.');
        renderHud();
        var fast = turnMs === 0 ? 0 : 80;
        Cube.scrambleMoves(20).forEach(function (move) {
            enqueue(move, false, fast);
        });
    }

    function undo() {
        if (scrambling || !history.length) return;
        idle = false;
        queue.push({ undo: true });
        pump();
    }

    viewport.addEventListener('pointerdown', function (event) {
        if (event.button !== 0 && event.button !== 2) return;
        idle = false;
        try { viewport.setPointerCapture(event.pointerId); } catch (err) { /* pointer already gone */ }
        var faceEl = event.target.closest ? event.target.closest('.face') : null;
        var cubie = faceEl && faceEl._cubie;
        var face = faceEl && faceEl.dataset.face;
        var twist = event.button === 0 && cubie && face && cubie.color[face];
        drag = {
            id: event.pointerId,
            x: event.clientX,
            y: event.clientY,
            mode: twist ? 'twist' : 'orbit',
            cubie: cubie,
            face: face,
            moved: false
        };
        viewport.classList.add('dragging');
        rig.style.transition = 'none';
    });

    viewport.addEventListener('pointermove', function (event) {
        if (!drag || event.pointerId !== drag.id) return;
        var dx = event.clientX - drag.x;
        var dy = event.clientY - drag.y;
        if (drag.mode === 'orbit') {
            yaw += dx * 0.45;
            pitch -= dy * 0.35;
            drag.x = event.clientX;
            drag.y = event.clientY;
            renderRig();
            return;
        }
        commitTwist(dx, dy);
    });

    function commitTwist(dx, dy) {
        if (!drag || drag.mode !== 'twist' || drag.moved) return;
        var normal = FACE_NORMAL[drag.face];
        if (Math.hypot(dx, dy) < 16) return;
        var world = Cube.screenToWorld(dx, dy, yaw, pitch);
        var along = world.x * normal.x + world.y * normal.y + world.z * normal.z;
        var plane = Math.hypot(world.x - normal.x * along, world.y - normal.y * along, world.z - normal.z * along);
        if (plane < Math.hypot(world.x, world.y, world.z) * 0.35) return;
        drag.moved = true;
        userMove(Cube.dragMove(normal, world, drag.cubie));
    }

    function endDrag(event) {
        if (!drag || event.pointerId !== drag.id) return;
        var dx = event.clientX - drag.x;
        var dy = event.clientY - drag.y;
        if (drag.mode === 'twist' && !drag.moved && Math.hypot(dx, dy) < 16) {
            var normal = FACE_NORMAL[drag.face];
            var axis = normal.x ? 'x' : normal.y ? 'y' : 'z';
            var layer = normal[axis];
            var base = null;
            Object.keys(Cube.FACE).forEach(function (key) {
                var face = Cube.FACE[key];
                if (face.axis === axis && face.layer === layer) base = face;
            });
            if (base) userMove({ axis: base.axis, layer: base.layer, dir: base.dir });
            drag.moved = true;
        } else {
            commitTwist(dx, dy);
        }
        drag = null;
        viewport.classList.remove('dragging');
    }

    viewport.addEventListener('pointerup', endDrag);
    viewport.addEventListener('pointercancel', endDrag);
    viewport.addEventListener('contextmenu', function (event) { event.preventDefault(); });

    document.getElementById('moves').addEventListener('click', function (event) {
        var button = event.target.closest('button');
        if (!button) return;
        userSeq(button.getAttribute('data-move'));
    });
    document.getElementById('scramble').addEventListener('click', scramble);
    undoBtn.addEventListener('click', undo);
    document.getElementById('restart').addEventListener('click', resetCube);

    document.addEventListener('keydown', function (event) {
        if (event.metaKey || event.ctrlKey || event.altKey) return;
        var key = event.key;
        if (key === 'ArrowLeft' || key === 'ArrowRight' || key === 'ArrowUp' || key === 'ArrowDown') {
            idle = false;
            rig.style.transition = 'transform 160ms ease';
            if (key === 'ArrowLeft') yaw -= 18;
            if (key === 'ArrowRight') yaw += 18;
            if (key === 'ArrowUp') pitch -= 14;
            if (key === 'ArrowDown') pitch += 14;
            renderRig();
            event.preventDefault();
            return;
        }
        if (event.repeat) return;
        if (key === 'Backspace') {
            undo();
            event.preventDefault();
            return;
        }
        if (key === ' ') {
            scramble();
            event.preventDefault();
            return;
        }
        var letter = key.toUpperCase();
        if (!Cube.FACE[letter]) return;
        event.preventDefault();
        var base = Cube.FACE[letter];
        userMove({
            axis: base.axis,
            layer: base.layer,
            dir: event.shiftKey ? -base.dir : base.dir
        });
    });

    window.cube = {
        twist: function (seq) {
            userSeq(seq);
            return seq;
        },
        inspect: function () {
            return Cube.inspect(state);
        }
    };

    mount();
    renderRig();
    renderHud();

    function frame(now) {
        if (idle && !drag) {
            yaw = -36 + Math.sin(now / 1600) * 10;
            pitch = -28 + Math.sin(now / 2100) * 4;
            renderRig();
        }
        window.requestAnimationFrame(frame);
    }
    window.requestAnimationFrame(frame);
}
