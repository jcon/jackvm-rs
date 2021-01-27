import { JackVirtualMachine } from "../pkg/index";

const HEIGHT = 256;
const WIDTH = 512;

const TICKS_PER_STEP = 24000;

// const parentEl = document.getElementById('screen-container');
// const mainCanvas = createCanvas(HEIGHT, WIDTH);
// parentEl.appendChild(mainCanvas);
// const progEl = document.querySelector("#editor");

function createCanvas(height, width) {
    const mainCanvas = document.createElement('canvas');
    mainCanvas.className = "screen";
    mainCanvas.height = height;
    mainCanvas.width = width;
    return mainCanvas;
}

class MemoryDebugger {
    constructor(vm) {
        this.vm = vm;
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
        let stackPointer = this.vm.peek(0);
        for (let i = 0; i < this.allMemoryCellIds.length - 1; i++) {
            let cellId = this.allMemoryCellIds[i];
            this.memoryCells[cellId].innerHTML = `${this.vm.peek(cellId)}${stackPointer === cellId ? " < SP" : ""}`;
        }
        this.memoryCells[24576].innerHTML = `${this.vm.peek(24575)} (key: ${String.fromCharCode(this.vm.peek(24575))})`;
    }

}

class Player {
    constructor(parentEl, config = { debugMemory: false }) {
        const canvas = createCanvas(HEIGHT, WIDTH);
        parentEl.appendChild(canvas);

        const screenBuffer = new ArrayBuffer(HEIGHT * WIDTH * 4);
        this.screenBytes = new Uint8Array(screenBuffer);
        this.vm = JackVirtualMachine.new(screenBuffer);
        this.imageData = new ImageData(WIDTH, HEIGHT);
        this.imageData.data.set(this.screenBytes);
        this.isPaused = true;
        this.isLoaded = false;
        this.mainContext = canvas.getContext('2d')
        if (config.debugMemory) {
            this.memoryDebugger = new MemoryDebugger(this.vm);
        }
    }

    drawScreen() {
        if (!this.isPaused) {
            requestAnimationFrame(this.drawScreen.bind(this));
        }
        this.vm.render_screen();
        this.imageData.data.set(this.screenBytes);
        this.mainContext.putImageData(this.imageData, 0, 0);
    }

    loadProgram(prog) {
        let result = this.vm.load(prog);
        if (!result.succeeded) {
            console.log("errors ****", result.get_errors());
        }
        this.isLoaded = true;
    }

    run() {
        if (!this.isPaused) {
            return;
        }

        this.isPaused = false;
        // if (!this.isLoaded) {
        //     this.loadProgram();
        // }

        this.imageData.data.set(this.screenBytes);
        this.mainContext.putImageData(this.imageData, 0, 0);

        this.executeSteps();
        this.drawScreen();
    }

    pause() {
        this.isPaused = true;
    }

    executeSteps() {
        if (!this.isPaused) {
            requestAnimationFrame(this.executeSteps.bind(this));
        }
        this.vm.tick_times(TICKS_PER_STEP);
        // for (let i = 0; i < TICKS_PER_STEP; i++) {
        //     this.vm.tick();
        // }
        this.memoryDebugger.update();
    }

    handleKeyDown(e) {
        e = e || window.event;

        let keyCode = e.keyCode;
        if (keyCode == 37) {
            keyCode = 130;
        }
        if (keyCode == 39) {
            keyCode = 132;
        }
        // console.log(`key pressed: ${e.keyCode} => ${keyCode}`);
        this.vm.set_key(keyCode);
    }

    handleKeyUp(e) {
        this.vm.set_key(0);
    }
}


export {
    Player
};