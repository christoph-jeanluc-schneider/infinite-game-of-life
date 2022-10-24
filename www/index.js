import { memory } from "infinite-game-of-life/infinite_game_of_life_bg";
import { init, Universe, Rules } from "infinite-game-of-life";

init();

const CELL_SIZE = 8;
const width = Math.round(window.innerWidth / CELL_SIZE);
const height = Math.round(window.innerHeight / CELL_SIZE);

const rules = {
	original: Rules.new([2, 3], [3]),
	labyrint: Rules.new([1, 2, 3, 4, 5], [3]),
};

const universe = Universe.new(width, height, rules.labyrint);

const canvas = document.getElementById("canvas");
canvas.width = window.innerWidth;
canvas.height = window.innerHeight;
const ctx = canvas.getContext("2d");

const getIndex = (row, column) => {
	return row * width + column;
};

const MIN_LIGHT = 2;
const DIV = (255 - MIN_LIGHT) / 100;
const lightness = (life) => {
	return Math.floor(life / DIV + MIN_LIGHT);
};

const renderLoop = () => {
	universe.tick();
	drawCells();

	setTimeout(() => requestAnimationFrame(renderLoop), 8);
};

const drawCells = () => {
	const cellsPtr = universe.cells();
	const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

	ctx.beginPath();

	for (let row = 0; row < height; row++) {
		for (let col = 0; col < width; col++) {
			const idx = getIndex(row, col);

			ctx.fillStyle = `hsl(0,0%,${lightness(cells[idx])}%)`;

			ctx.fillRect(
				col * CELL_SIZE,
				row * CELL_SIZE,
				CELL_SIZE,
				CELL_SIZE
			);
		}
	}

	ctx.stroke();
};

drawCells();
requestAnimationFrame(renderLoop);
