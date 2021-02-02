import { JackVirtualMachine, greet } from "jackvm-player";
import { MemoryDebugger } from './memory-debugger';

const HEIGHT = 256;
const WIDTH = 512;

const TICKS_PER_STEP = 40000;


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


    loadProgram(prog) {
        let result = this.vm.load(prog);
        if (!result.succeeded) {
            const message = `JackVmPlayer could not load program due to the following errors:\n\n${result.get_errors().join("\n")}`;
            alert(message);
            return;
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
            return;
        }

        this.isPaused = false;

        this.imageData.data.set(this.screenBytes);
        this.mainContext.putImageData(this.imageData, 0, 0);

        this.nextFrame();
    }

    pause() {
        this.isPaused = true;
    }

    executeSteps() {
        this.vm.tick_times(TICKS_PER_STEP);
    }

    drawScreen() {
        this.vm.render_screen();
        this.imageData.data.set(this.screenBytes);
        this.mainContext.putImageData(this.imageData, 0, 0);
    }

    nextFrame() {
        if (!this.isPaused && !this.vm.isHalted()) {
            requestAnimationFrame(this.nextFrame.bind(this));
        } else {
            this.handleHalt();
        }

        this.executeSteps();
        this.drawScreen();

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