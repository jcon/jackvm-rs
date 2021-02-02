// import * as wasm from "jackvm-player";
import { JackVirtualMachine, greet } from "jackvm-player";
import { MemoryDebugger } from './memory-debugger';

const HEIGHT = 256;
const WIDTH = 512;

const TICKS_PER_STEP = 40000;

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


class Player {
    constructor(parentEl, config = { debugMemory: false }) {
        const canvas = createCanvas(HEIGHT, WIDTH);
        parentEl.appendChild(canvas);
        this.canvas = canvas;

        const screenBuffer = new ArrayBuffer(HEIGHT * WIDTH * 4);
        this.screenBytes = new Uint8Array(screenBuffer);
        this.vm = JackVirtualMachine.new(screenBuffer);
        this.imageData = new ImageData(WIDTH, HEIGHT);
        this.imageData.data.set(this.screenBytes);
        this.isPaused = true;
        this.isLoaded = false;
        this.mainContext = canvas.getContext('2d');
        this.haltListeners = [];
        if (config.debugMemory) {
            this.memoryDebugger = new MemoryDebugger(this.vm);
        }
    }

    getCanvas() {
        return this.canvas;
    }

    drawScreen() {
        if (!this.isPaused && !this.vm.isHalted()) {
            requestAnimationFrame(this.drawScreen.bind(this));
        } else {
            // console.log('VM is halted, no more screen refreshes.');
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

    isHalted() {
        return this.vm.isHalted();
    }

    restart() {
        this.isPaused = true;
        this.vm.restart();
        this.run();
    }

    run() {
        if (!this.isPaused) {
            return;``
        }

        this.isPaused = false;
        // if (this.vm.isHalted()) {
        //     console.log("VM was halted, restarting");
        //     this.vm.restart();
        // }
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
        if (!this.isPaused && !this.vm.isHalted()) {
            requestAnimationFrame(this.executeSteps.bind(this));
        } else {
            this.handleHalt();
            // console.log('VM is halted, no VM ticks.');
        }

        this.vm.tick_times(TICKS_PER_STEP);
        // for (let i = 0; i < TICKS_PER_STEP; i++) {
        //     this.vm.tick();
        // }
        if (this.memoryDebugger) {
            this.memoryDebugger.update();
        }
    }

    addHaltListener(f) {
        this.haltListeners.push(f);
    }

    handleHalt() {
        this.haltListeners.forEach(f => f());
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
    Player as JackVmPlayer
};