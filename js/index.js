import { JackVirtualMachine } from "../pkg/index";

const screenBuffer = new ArrayBuffer(512 * 256 * 4);
const screenBytes = new Uint8Array(screenBuffer);
const screenBytes32 = new Uint32Array(screenBuffer);
const vm = JackVirtualMachine.new(screenBuffer);

const mainCanvas = document.getElementById('screen');
mainCanvas.height = 256;
mainCanvas.width = 512;
const mainContext = mainCanvas.getContext('2d');
const imageData = new ImageData(512, 256);
imageData.data.set(screenBytes);

let progEl = document.querySelector("#editor");
console.log(progEl.value.split('\n').map(s => s.trim()));


class MemoryDebugger {
    constructor() {
        let registerCellIds = [0, 1, 2, 3, 4]; // 5, 6, 7, 8];
        let memoryCellIds = [256, 257, 258, 259, 260, 261, 262, 24576]; // , 16384, 16416, 16448];

        this.memoryCells = {};
        this.initializedMemory = false;
        this.allMemoryCellIds = [];
        let tableBody = document.querySelector("#memory-body");
        if (tableBody !== undefined) {
            for (let i = 0; i < memoryCellIds.length; i++) {
                let tableRow = document.createElement("tr");
                let rowHead = document.createElement("th");
                rowHead.innerHTML = memoryCellIds[i].toString();
                tableRow.appendChild(rowHead);
                let rowCell = document.createElement("td");
                rowCell.innerHTML = "0";
                tableRow.appendChild(rowCell);
                tableBody.appendChild(tableRow);
                let cellId = memoryCellIds[i];

                this.memoryCells[cellId] = rowCell;
            }
            let that = this;
            registerCellIds.forEach(cellId => {
                that.memoryCells[cellId] = document.querySelector(`#mem-${cellId}`);
            });

            this.allMemoryCellIds = registerCellIds.concat(memoryCellIds);
            this.initializedMemory = true;
        }
    }

    update() {
        if (!this.initializedMemory) {
            return;
        }
        let stackPointer = vm.peek(0);
        for (let i = 0; i < this.allMemoryCellIds.length - 1; i++) {
            let cellId = this.allMemoryCellIds[i];
             this.memoryCells[cellId].innerHTML = `${vm.peek(cellId)}${stackPointer === cellId ? " < SP" : ""}`;
        }
        this.memoryCells[24576].innerHTML = `${vm.peek(24575)} (key: ${String.fromCharCode(vm.peek(24575))})`;
    }

}

let memoryDebugger = new MemoryDebugger();


document.onkeydown = function (e) {
    e = e || window.event;

    let keyCode = e.keyCode;
    if (keyCode == 37) {
        keyCode = 130;
    }
    if (keyCode == 39) {
        keyCode = 132;
    }
    // console.log(`key pressed: ${e.keyCode} => ${keyCode}`);
    vm.set_key(keyCode);
    // use e.keyCode
};

document.onkeyup = function (e) {
    vm.set_key(0);
};

function dec2bin(dec){
   return (dec >>> 0).toString(2).padStart(16, "0");
}

let isPaused = false;

function drawScreen() {
    if (!isPaused) {
        requestAnimationFrame(drawScreen);
    }
    vm.render_screen();
    imageData.data.set(screenBytes);
    mainContext.putImageData(imageData, 0, 0);
}

const ticksPerStep = 30000;
function executeSteps() {
    if (!isPaused) {
        requestAnimationFrame(executeSteps);
    }
    for (let i = 0; i < ticksPerStep; i++) {
        vm.tick();
    }
    memoryDebugger.update();
}

let isLoaded = false;

progEl.addEventListener("change", event => {
    isLoaded = false;
});

let runEl = document.querySelector("#run");
runEl.addEventListener("click", event => {
    isPaused = false;
    if (!isLoaded) {
        const prog = progEl.value.split('\n').map(s => s.trim());
        // console.log("loading program [", prog.join("\n"), "]");
        let result = vm.load(prog.join("\n"));
        if (!result.succeeded) {
            console.log("errors ****", result.get_errors());
        }
        isLoaded = true;
    }

    imageData.data.set(screenBytes);
    mainContext.putImageData(imageData, 0, 0);

    executeSteps();
    drawScreen();
});

let pauseEl = document.querySelector("#pause");
pauseEl.addEventListener("click", event => {
    isPaused = true;
});
