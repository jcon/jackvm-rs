// console.log("Elevenpack javascript is loaded");
// import { JackVmPlayer } from './player';
import { JackVmPlayer } from "jackvm-player";
const parentEl = document.getElementById('screen-container');

let player = new JackVmPlayer(parentEl, { onColor: 0xffffffff, offColor: 0x000000ff }); // , { debugMemory: false });
let canvas = parentEl.querySelector('canvas');
// Support two modes depending on the tailwind breakpoint reached:
// 1x or 2x
['sm:w-512px', 'sm:h-256px', 'lg:w-1024px', 'lg:h-512px'].forEach(cls => {
  canvas.classList.add(cls);
});

// Update the VM's program whenever the "editor" changes.
const progEl = document.querySelector("#editor");
progEl.addEventListener("change", event => {
    updateProgram();
});

// Start running the VM if it's not already running.
let overlayEl = document.querySelector("#play-overlay");
overlayEl.addEventListener("click", event => {
    console.log('player clicked');
    // if (player.isPaused || player.isHalted()) {
    if (player.isStopped()) {
      player.restart();
      overlayEl.classList.toggle("hidden");
    }
    return false;
});

player.addHaltListener(() => {
  overlayEl.classList.toggle("hidden");
});

window.dispatchEvent(new CustomEvent('JackVmPlayerLoaded', {
  detail: player,
}));