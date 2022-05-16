import init, * as wasm from "./wasm";

export const draw = (raster) => {
  console.log(raster.pixels());
};

await init();

const raster = new wasm.Raster(1600, 1600);

const canvas = document.getElementById("canvas");
/** @type {CanvasRenderingContext2D} */
const ctx = canvas.getContext("2d");
console.log(raster.pixels());
window.raster=raster;
const imageData = new ImageData(
  new Uint8ClampedArray(raster.pixels()),
  raster.height()
);

window.ctx=ctx;
ctx.putImageData(imageData, 0, 0);
