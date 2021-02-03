import { JackVirtualMachine, greet } from "jackvm-player";
import { MemoryDebugger } from './memory-debugger';

const HEIGHT = 256;
const WIDTH = 512;

const TICKS_PER_STEP = 40000;


// function createCanvas(height, width) {
//     const mainCanvas = document.createElement('canvas');
//     mainCanvas.className = "screen";
//     mainCanvas.height = height;
//     mainCanvas.width = width;
//     return mainCanvas;
// }

class Player {
    constructor(parentEl, config = { debugMemory: false }) {
        // const canvas = createCanvas(HEIGHT, WIDTH);
        // parentEl.appendChild(canvas);
        // this.canvas = canvas;

        const screenBuffer = new ArrayBuffer(HEIGHT * WIDTH * 4);
        this.screenBytes = new Uint8Array(screenBuffer);
        this.imageData = new ImageData(WIDTH, HEIGHT);
        // this.imageData.data.set(this.screenBytes);

        this.vm = JackVirtualMachine.new(screenBuffer, parentEl);
        this.canvas = parentEl.querySelector('canvas');

        this.mainContext = this.canvas.getContext('2d');
        this.vm.setIsPaused(true);
        // this.isPaused = true;
//        this.isLoaded = false;
        this.haltListeners = [];
        if (config.debugMemory) {
            this.memoryDebugger = new MemoryDebugger(this.vm);
        }
    }

    getCanvas() {
        return this.canvas;
    }


    loadProgram(prog) {
        this.vm.load(prog);
        // let result = this.vm.load(prog);
        // if (!result.succeeded) {
        //     const message = `JackVmPlayer could not load program due to the following errors:\n\n${result.get_errors().join("\n")}`;
        //     window.alert(message);
        //     return;
        // }
//        this.isLoaded = true;
    }

    isHalted() {
        return this.vm.isHalted();
    }

    isStopped() {
        return this.vm.isStopped();
    }

    restart() {
        // this.isPaused = true;
        this.vm.setIsPaused(true);
        this.vm.restart();
        this.run();
    }

    run() {
        console.log('running');
        //if (!this.isPaused) {
        if (!this.vm.isPaused()) {
            return;
        }

        // this.isPaused = false;
        this.vm.setIsPaused(false);

        this.copyScreen();
        this.nextFrame();
    }

    copyScreen() {
        // this.imageData.data.set(this.screenBytes);
        // this.mainContext.putImageData(this.imageData, 0, 0);
        this.vm.copyScreen();
    }

    pause() {
        // this.isPaused = true;
        this.vm.setIsPaused(true);
    }

    // executeSteps() {
    //     this.vm.tick_times(TICKS_PER_STEP);
    // }

    // drawScreen() {
    //     this.vm.render_screen();
    //     this.copyScreen();
    // }

    nextFrame() {
        // if (!this.isPaused && !this.vm.isHalted()) {
        if (!this.vm.isStopped()) {
            window.requestAnimationFrame(this.nextFrame.bind(this));
        } else {
            this.handleHalt();
        }

        this.vm.nextFrame();
        // this.vm.executeSteps();
        // this.drawScreen();

        // if (this.memoryDebugger) {
        //     this.memoryDebugger.update();
        // }
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