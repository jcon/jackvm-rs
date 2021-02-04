import { JackVmPlayer, greet } from "jackvm-player";
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

        // SAVE FOR SCREEN TESTS.
        // const screenBuffer = new ArrayBuffer(HEIGHT * WIDTH * 4);
        // this.screenBytes = new Uint8Array(screenBuffer);
        // this.imageData = new ImageData(WIDTH, HEIGHT);
        // this.imageData.data.set(this.screenBytes);

        this.vm = JackVmPlayer.new(parentEl);

        // SAVE FOR SCREEN TESTS.
        // this.mainContext = this.canvas.getContext('2d');
        this.vm.setIsPaused(true);
        // this.isPaused = true;
//        this.isLoaded = false;
        // this.haltListeners = [];
        if (config.debugMemory) {
            this.memoryDebugger = new MemoryDebugger(this.vm);
        }
    }

    // ----- public methods -----
    load(prog) {
        this.vm.load(prog);
    }

    isStopped() {
        return this.vm.isStopped();
    }

    restart() {
        this.vm.restart();
        // this.vm.setIsPaused(true);
        // this.vm.restart();
        // this.run();
    }

    addHaltListener(f) {
        this.vm.addHaltListener(f);
    }


    // isHalted() {
    //     return this.vm.isHalted();
    // }



    // ----- private methods -----
    // run() {
    //     this.vm.run();
    //     // console.log('running');
    //     // if (!this.vm.isPaused()) {
    //     //     return;
    //     // }

    //     // this.vm.setIsPaused(false);

    //     // this.copyScreen();
    //     // this.nextFrame();
    // }

    // SAVE FOR SCREEN TESTS
    // copyScreen() {
    //     // this.imageData.data.set(this.screenBytes);
    //     // this.mainContext.putImageData(this.imageData, 0, 0);
    //     this.vm.copyScreen();
    // }

    // pause() {
    //     this.vm.setIsPaused(true);
    // }

    // executeSteps() {
    //     this.vm.tick_times(TICKS_PER_STEP);
    // }

    // drawScreen() {
    //     this.vm.render_screen();
    //     this.copyScreen();
    // }

    // nextFrame() {
    //     if (!this.vm.isStopped()) {
    //         window.requestAnimationFrame(this.nextFrame.bind(this));
    //     } else {
    //         this.vm.handleHalt();
    //     }

    //     this.vm.nextFrame();
    // }


    // ----- event listeners -----
    // handleKeyDown(e) {
    //     e = e || window.event;

    //     this.vm.handleKeyDown(e);
    //     // let keyCode = e.keyCode;
    //     // if (keyCode == 37) {
    //     //     keyCode = 130;
    //     // }
    //     // if (keyCode == 39) {
    //     //     keyCode = 132;
    //     // }
    //     // // console.log(`key pressed: ${e.keyCode} => ${keyCode}`);
    //     // this.vm.set_key(keyCode);
    // }

    // handleKeyUp(e) {
    //     // this.vm.set_key(0);
    //     this.vm.handleKeyUp();
    // }
}

export {
    Player as JackVmPlayer
};