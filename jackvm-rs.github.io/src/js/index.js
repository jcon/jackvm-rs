// console.log("Elevenpack javascript is loaded");
import { JackVmPlayer } from './player';

const parentEl = document.getElementById('screen-container');

let player = new JackVmPlayer(parentEl, { debugMemory: false });
// TODO: bind these automatically.
document.onkeydown = player.handleKeyDown.bind(player);
document.onkeyup = player.handleKeyUp.bind(player);

// Update the VM's program whenever the "editor" changes.
const progEl = document.querySelector("#editor");
progEl.addEventListener("change", event => {
    updateProgram();
});

// Start running the VM if it's not already running.
let runEl = document.querySelector("#play-overlay");
runEl.addEventListener("click", event => {
    if (player.isPaused || player.isHalted()) {
      player.restart();
      runEl.classList.toggle("hidden");
    }
    return false;
});

player.addHaltListener(() => {
  runEl.classList.toggle("hidden");
});

// Pause the VM if it's not already paused.
// let pauseEl = document.querySelector("#screen-overlay canvas");
// pauseEl.addEventListener("click", event => {
//     if (!player.isPaused && !player.isHalted()) {
//       player.pause();
//     }
// });

// example loading vm program concatenated.
// fetch('vms/pong.vm')
//   .then(res => res.text())
//   .then(program => {
//     progEl.value = program;
//     updateProgram();
//   });

// // Helper to update the current program being run by the VM.
// function updateProgram() {
//     const program = progEl.value.split('\n').map(s => s.trim()).join("\n");
//     player.loadProgram(program);
// }

// updateProgram();

// window.vmPlayer = player;

window.dispatchEvent(new CustomEvent('JackVmPlayerLoaded', {
  detail: player,
}));