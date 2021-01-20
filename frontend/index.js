import { JackVirtualMachine } from "jackvm-wasm";

const screenBuffer = new ArrayBuffer(512 * 256 * 4);
const screenBytes = new Uint8Array(screenBuffer);
const vm = JackVirtualMachine.new(screenBuffer);

const mainCanvas = document.getElementById('screen');
const mainContext = mainCanvas.getContext('2d');
const imageData = new ImageData(256, 512);
imageData.data.set(screenBytes);

let progEl = document.querySelector("#editor");
console.log(progEl.value.split('\n').map(s => s.trim()));

let runEl = document.querySelector("#run");

let memoryCells = {};
let memoryCellIds = [0, 1, 2, 256, 257, 258, 259, 260, 261, 262, 16384, 16416, 16448, 24575]
for (let i = 0; i < memoryCellIds.length; i++) {
    let cellId = memoryCellIds[i];
    memoryCells[cellId] = document.querySelector(`#mem-${cellId}`);
}
// for (let i = 0; i < 3; i++) {
//     memoryCells[i] = document.querySelector(`#mem-${i}`);
// }
// for (let i = 256; i < 262; i++) {
//     memoryCells[i] = document.querySelector(`#mem-${i}`);
// }

(function drawScreen() {
    requestAnimationFrame(drawScreen);
    vm.render_screen();
    imageData.data.set(screenBytes);
    mainContext.putImageData(imageData, 0, 0);
})();

document.onkeypress = function (e) {
    e = e || window.event;
    console.log(e.keyCode);
    vm.set_key(e.keyCode);
    // use e.keyCode
};

document.onkeyup = function (e) {
    vm.set_key(0);
};

runEl.addEventListener("click", event => {
    const prog = progEl.value.split('\n').map(s => s.trim());
    // console.log("loading program [", prog.join("\n"), "]");
    let result = vm.load(prog.join("\n"));
    if (!result.succeeded) {
        console.log("errors ****", result.get_errors());
    }

    imageData.data.set(screenBytes);
    mainContext.putImageData(imageData, 0, 0);

    let frames = 0;

    (function executeSteps() {
    //    if (frames < 8) {
        // if (frames < 512) {
            requestAnimationFrame(executeSteps);
        // }
        frames++;
        3000
        for (let i = 0; i < 8000; i++) {
            vm.tick();
        }
        console.log("**** value from VM: " + vm.peek(256));
        // for (let i = 0; i < 3; i++) {
        //     memoryCells[i].innerHTML = `${vm.peek(i)}`;
        // }
        let stackPointer = vm.peek(0);
        // for (let i = 256; i < 262; i++) {
        //     memoryCells[i].innerHTML = `${vm.peek(i)}${stackPointer === i ? " < SP" : ""}`;
        // }
        for (let i = 0; i < memoryCellIds.length; i++) {
            let cellId = memoryCellIds[i];
            memoryCells[cellId].innerHTML = `${vm.peek(cellId)}${stackPointer === cellId ? " < SP" : ""}`;
        }
    })();
});


