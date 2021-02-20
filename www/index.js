import * as wasm from "rust-wasm";

wasm.greet();

const canvas = document.getElementById("appCanvas");
const gl = canvas.getContext("webgl", { antialias: true});

if (!gl) {
    alert('Failed to initialize WebGL');
}

const FPS_THROTTLE = 1000.0 / 30.0; // milliseconds / frames
