import { verify, Universe, Cell/*, AppClient*/ } from 'rust-wasm'
import { memory } from '../pkg/rust_wasm_bg'

verify()

// build game Universe
const universe = Universe.new()
const width = universe.width()
const height = universe.height()

// configure display settings
const CELL_SIZE = 40 // px
const GRID_COLOUR = '#000000'
const EMPTY_COLOUR = '#000000'
const BLOCK_COLOUR = '#E69E19'
const P1_COLOUR = '#0000FF'
const P2_COLOUR = '#FF0000'

// setup canvas
const gameCanvas = document.getElementById('game')
gameCanvas.height = (CELL_SIZE + 1) * height + 1
gameCanvas.width = (CELL_SIZE + 1) * width + 1

const ctx = gameCanvas.getContext('2d')

const renderLoop = () => {
  // universe.tick()
  drawGrid()
  drawCells()
  // requestAnimationFrame(renderLoop)
}

const drawGrid = () => {
  ctx.beginPath()
  ctx.strokeStyle = GRID_COLOUR
  // Vertical lines
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0)
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1)
  }
  // Horizontal lines
  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1)
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1)
  }

  ctx.stroke()
}

const getIndex = (row, column) => {
  return row * width + column
}

const drawCells = () => {
  const cellsPtr = universe.cells()
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height)
  ctx.beginPath()

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col)
      switch (cells[idx]) {
        case Cell.Block:
          ctx.fillStyle = BLOCK_COLOUR
          break
        case Cell.Player1:
          ctx.fillStyle = P1_COLOUR
          break
        case Cell.Player2:
          ctx.fillStyle = P2_COLOUR
          break
        default:
          ctx.fillStyle = EMPTY_COLOUR
      }
      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      )
    }
  }
  ctx.stroke()
}

renderLoop()

//
//
// WebGL stuff from here, come back to this later
//
//
// const canvas = document.getElementById('app-canvas')
// const gl = canvas.getContext('webgl', { antialias: true})
// if (!gl) {
//   console.log('Failed to initialize WebGL')
// }
// const FPS_THROTTLE = 1000.0 / 1.0 // 30 fps - for now leave at 1 to reduce impact during initial setup
// const app = new AppClient()
// const initialTime = Date.now()
// let lastDrawTime = -1 // In milliseconds
// function render() {
//   window.requestAnimationFrame(render)
//   const currTime = Date.now()
//   // implement throttling
//   if (currTime >= lastDrawTime + FPS_THROTTLE) {
//     lastDrawTime = currTime
//     // support resizing of window
//     if (window.innerHeight !== canvas.height || window.innerWidth !== canvas.width) {
//       canvas.height = window.innerHeight
//       canvas.style.height = window.innerHeight
//       canvas.width = window.innerWidth
//       canvas.style.width = window.innerWidth
//       gl.viewport(0, 0, window.innerWidth, window.innerHeight)
//     }
//     let elapsedTime = currTime - initialTime
//     app.update(elapsedTime, window.innerHeight, window.innerWidth)
//     app.render()
//   }
// }
// render()
