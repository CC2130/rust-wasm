// Import the WebAssembly memory at the top of the file.
// wasm-game-of-life/wasm_game_of_life_bg.wasm
import { Cell, Universe } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

const CELL_SIZE = 5;
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const universe = Universe.new();
const width = universe.width();
const height = universe.height();

let animationid = null;
const canvas = document.getElementById("game-of-life-canvas");
const reset = document.getElementById("reset");
const playPauseButton = document.getElementById("play-pause");
const start = document.getElementById("start");
const ctx = canvas.getContext("2d");

canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

reset.addEventListener("click", function () {
  universe.reset();
  drawGrid();
  drawCells();
  pause();
});

start.addEventListener("click", function () {
  universe.start();
  drawGrid();
  drawCells();
  requestAnimationFrame(renderLoop);
});

function isPaused() {
  return animationid === null;
}

function play() {
  console.log("Start animation!");
  playPauseButton.textContent = "⏸";
  renderLoop();
}

function pause() {
  console.log("Stop animation!");
  playPauseButton.textContent = "▶";
  cancelAnimationFrame(animationid);
  animationid = null;
}

playPauseButton.addEventListener("click", function () {
  if (isPaused()) {
    play();
  } else {
    pause();
  }
});

function renderLoop() {
  universe.tick();

  drawGrid();
  drawCells();

  animationid = requestAnimationFrame(renderLoop);
}

function drawGrid() {
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

  ctx.stroke();
}

function getIndex(row, column) {
  return row * width + column;
}

function drawCells() {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);

      ctx.fillStyle = cells[idx] === Cell.Dead ? DEAD_COLOR : ALIVE_COLOR;

      ctx.fillRect(col * (CELL_SIZE + 1) + 1, row * (CELL_SIZE + 1) + 1, CELL_SIZE, CELL_SIZE);
    }
  }

  ctx.stroke();
}

canvas.addEventListener("click", function (event) {
  const boundingRect = canvas.getBoundingClientRect();

  // 换算比例
  const scaleX = canvas.width / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;

  // canvas 中对应的座标
  const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
  const canvasTop = (event.clientY - boundingRect.top) * scaleY;

  // 计算行列
  const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
  const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

  universe.toggle_cell(row, col);

  drawGrid();
  drawCells();
});

drawGrid();
drawCells();
requestAnimationFrame(renderLoop);
