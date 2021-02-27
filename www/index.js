import {verify, Universe, Cell, AppClient} from "rust-wasm";

verify();

const pre = document.getElementById("game");
const universe = Universe.new();
pre.textContent = universe.render();


// WebGL stuff from here, come back to this later 

const canvas = document.getElementById("appCanvas");
const gl = canvas.getContext("webgl", { antialias: true});

if (!gl) {
    console.log("Failed to initialize WebGL");
}

const FPS_THROTTLE = 1000.0 / 1.0; // 30 fps - for now leave at 1 to reduce impact during initial setup
const app = new AppClient();
const initialTime = Date.now();
let lastDrawTime = -1;// In milliseconds

function render() {
    window.requestAnimationFrame(render);
    const currTime = Date.now();

    // implement throttling
    if (currTime >= lastDrawTime + FPS_THROTTLE) {
        lastDrawTime = currTime;

        // support resizing of window
        if (window.innerHeight !== canvas.height || window.innerWidth !== canvas.width) {
            canvas.height = window.innerHeight;
            //canvas.clientHeight = window.innerHeight;
            canvas.style.height = window.innerHeight;

            canvas.width = window.innerWidth;
            //canvas.clientWidth = window.innerWidth;
            canvas.style.width = window.innerWidth;

            gl.viewport(0, 0, window.innerWidth, window.innerHeight);
        }

        let elapsedTime = currTime - initialTime;
        app.update(elapsedTime, window.innerHeight, window.innerWidth);
        app.render();
    }
}

render();