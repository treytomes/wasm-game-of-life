import { init, Pattern, Universe } from 'wasm-game-of-life';

const CELL_SIZE = 5; // px
const GRID_COLOR = '#CCCCCC';
const DEAD_COLOR = '#FFFFFF';
const ALIVE_COLOR = '#000000';

init();

const SPACESHIP = Pattern.from_str(`
    001110
    011111
    110111
    011000
`);

// Construct the universe, then get it's size.
const universe = Universe.new(128, 128);
const width = universe.width();
const height = universe.height();
// universe.randomize();
SPACESHIP.place(universe, 64, 15);

// Give the canvas room for all the cells and 1px border around each of them.
const canvas = document.getElementById('game-of-life-canvas');
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

const renderLoop = () => {
    universe.tick();

    drawGrid();
    drawCells();
    requestAnimationFrame(renderLoop);
    debugger;
}

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // Vertical lines.
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke()
}

const drawCells = () => {
    ctx.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const cell = universe.get_cell(row, col);
            ctx.fillStyle = cell ? ALIVE_COLOR : DEAD_COLOR;
            ctx.fillRect(col * (CELL_SIZE + 1) + 1, row * (CELL_SIZE + 1) + 1, CELL_SIZE, CELL_SIZE);
        }
    }

    ctx.stroke();
}

drawGrid();
drawCells();
requestAnimationFrame(renderLoop);