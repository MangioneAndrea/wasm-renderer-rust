import init, * as wasm from "./wasm";

export const draw = (ruster) => {
  console.log(ruster.pixels());
};

await init();

const ruster = new wasm.Ruster(800, 800);

const canvas = document.getElementById("canvas");
/** @type {CanvasRenderingContext2D} */
const ctx = canvas.getContext("2d");
console.log(ruster.pixels().length);
const imageData = new ImageData(
  new Uint8ClampedArray(ruster.pixels()),
  ruster.height(),
  ruster.width()
);
ctx.putImageData(imageData, 0, 0);
