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
for (let i = 256; i < 262; i++) {
    memoryCells[i] = document.querySelector(`#mem-${i}`);
}

(function drawScreen() {
    requestAnimationFrame(drawScreen);
    vm.render_screen();
})();

runEl.addEventListener("click", event => {
    const prog = progEl.value.split('\n').map(s => s.trim());
    // console.log("loading program [", prog.join("\n"), "]");
    let result = vm.load(prog.join("\n"));
    if (!result.succeeded) {
        console.log("errors ****", result.get_errors());
    }

    imageData.data.set(screenBytes);
    mainContext.putImageData(imageData, 0, 0);

    (function executeSteps() {
        requestAnimationFrame(executeSteps);
        for (let i = 0; i < 150; i++) {
            vm.tick();
        }
        console.log("**** value from VM: " + vm.peek(256));
        let stackPointer = vm.peek(0);
        for (let i = 256; i < 262; i++) {
            memoryCells[i].innerHTML = `${vm.peek(i)}${stackPointer === i ? " < SP" : ""}`;
        }
    })();
});


