// console.log("Elevenpack javascript is loaded");
import { JackVmPlayer } from './player';

const parentEl = document.getElementById('screen-container');

let player = new JackVmPlayer(parentEl, { debugMemory: false });
// Support two modes depending on the tailwind breakpoint reached:
// 1x or 2x
['sm:w-512px', 'sm:h-256px', 'lg:w-1024px', 'lg:h-512px'].forEach(cls => {
  player.getCanvas().classList.add(cls);
});

// TODO: bind these automatically.
document.onkeydown = player.handleKeyDown.bind(player);
document.onkeyup = player.handleKeyUp.bind(player);

// Update the VM's program whenever the "editor" changes.
const progEl = document.querySelector("#editor");
progEl.addEventListener("change", event => {
    updateProgram();
});

// Start running the VM if it's not already running.
let overlayEl = document.querySelector("#play-overlay");
overlayEl.addEventListener("click", event => {
    if (player.isPaused || player.isHalted()) {
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