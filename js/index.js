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

let runEl = document.querySelector("#run");

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

    createCell(cellId) {

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

(function drawScreen() {
    requestAnimationFrame(drawScreen);
    // screenBytes.fill(0x000000FF);
    vm.render_screen();
    imageData.data.set(screenBytes);
    mainContext.putImageData(imageData, 0, 0);

    // mainContext.fillStyle = "rgba(0,0,0,1)";
    // for (let y = 16; y < 32; y++) {
    //     for (let x = 0; x < 16; x++) {
    //         if ((x + 16) == y) {
    //             mainContext.fillRect(x, y, 1, 1);
    //         }
    //     }
    // }

    // let y = 48;
    // for (let x = 0; x < 200; x++) {
    //     if (x % 2 === 0) {
    //         mainContext.fillRect(x, y, 1, 1);
    //     }
    // }
})();

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

    let debugIns = false;
    const ticksPerStep = 30000;
    (function executeSteps() {
    //    if (frames < 8) {
        // if (frames < 512) {
            requestAnimationFrame(executeSteps);
        // }
        frames++;
//        30000i

        for (let i = 0; i < ticksPerStep; i++) {
            // let ins = vm.get_instruction();
            // let stack_val = vm.peek(vm.peek(0) - 1);
            // if (ins === "Call(\"Main.main\", 0)") {
            //     debugIns = true;
            //     debugger;
            // }
            if (debugIns) {
                // console.log(ins);
            }
            vm.tick();
        }
        // updateMemory();
        memoryDebugger.update();
    })();

    function updateMemory() {
        // console.log("**** value from VM: " + vm.peek(256));
        // for (let i = 0; i < 3; i++) {
        //     memoryCells[i].innerHTML = `${vm.peek(i)}`;
        // }
        let stackPointer = vm.peek(0);
        // for (let i = 256; i < 262; i++) {
        //     memoryCells[i].innerHTML = `${vm.peek(i)}${stackPointer === i ? " < SP" : ""}`;
        // }
        for (let i = 0; i < allMemoryCellIds.length - 1; i++) {
            let cellId = allMemoryCellIds[i];
//           memoryCells[cellId].innerHTML = `${dec2bin(vm.peek(cellId))}${stackPointer === cellId ? " < SP" : ""}`;
             memoryCells[cellId].innerHTML = `${vm.peek(cellId)}${stackPointer === cellId ? " < SP" : ""}`;
        }
        memoryCells[24576].innerHTML = `${vm.peek(24575)} (key: ${String.fromCharCode(vm.peek(24575))})`;
    }
});
