import { JackVirtualMachine } from "jackvm-wasm";

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

let memoryCells = {};
let memoryCellIds = [0, 1, 2, 3, 4, 5, 6, 7, 8, 256, 257, 258, 259, 260, 261, 262]; // , 16384, 16416, 16448];
// // let bitmapStart = 16384+31;
// let bitmapStart = 16384+32; // + 7648 + 31;
// for (let i = 0; i < 10; i++) {
//     memoryCellIds.push(bitmapStart + 32*i);
//     memoryCellIds.push(bitmapStart + 32*i+1);
// }
memoryCellIds.push(24575);
let tableBody = document.querySelector("#memory-body");
for (let i = 0; i < memoryCellIds.length; i++) {
    /*         <tr>
    <th>SP</th>
    <td id="mem-0">0</td>
  </tr
  */
    let tableRow = document.createElement("tr");
    let rowHead = document.createElement("th");
    rowHead.innerHTML = memoryCellIds[i].toString();
    tableRow.appendChild(rowHead);
    let rowCell = document.createElement("td");
    rowCell.innerHTML = "0";
    tableRow.appendChild(rowCell);
    tableBody.appendChild(tableRow);
    let cellId = memoryCellIds[i];
    // memoryCells[cellId] = document.querySelector(`#mem-${cellId}`);
    memoryCells[cellId] = rowCell;
}
// for (let i = 0; i < 3; i++) {
//     memoryCells[i] = document.querySelector(`#mem-${i}`);
// }
// for (let i = 256; i < 262; i++) {
//     memoryCells[i] = document.querySelector(`#mem-${i}`);
// }

function registerMemoryPeek(id) {
    let addr = document.querySelector(`#memory-peek-address-${id}`);
    let val = vm.peek(addr);
    let valDec = document.querySelector(`#memory-peek-value-dec-${id}`);
    let valBin = document.querySelector(`#memory-peek-value-bin-${id}`);
    let updateButton = document.querySelector(`#memory-peek-update-${id}`);

    function updateValue() {
        let value = vm.peek(addr.value);
        valDec.innerHTML = value.toString();
        valBin.innerHTML = dec2bin(value.toString());
    }
    updateButton.addEventListener('click', event => {
        updateValue();
    });
    updateValue();
}

registerMemoryPeek(0);
registerMemoryPeek(1);
registerMemoryPeek(2);

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

document.onkeypress = function (e) {
    e = e || window.event;
    // console.log(e.keyCode);
    vm.set_key(e.keyCode);
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
    (function executeSteps() {
    //    if (frames < 8) {
        // if (frames < 512) {
            requestAnimationFrame(executeSteps);
        // }
        frames++;
//        30000i
        for (let i = 0; i < 8000; i++) {
            let ins = vm.get_instruction();
            if (ins === "Call(\"Output.drawChar\", 1)") {
                debugIns = true;
                debugger;
            }
            if (debugIns) {
                console.log(ins);
            }
            vm.tick();
        }
        // console.log("**** value from VM: " + vm.peek(256));
        // for (let i = 0; i < 3; i++) {
        //     memoryCells[i].innerHTML = `${vm.peek(i)}`;
        // }
        let stackPointer = vm.peek(0);
        // for (let i = 256; i < 262; i++) {
        //     memoryCells[i].innerHTML = `${vm.peek(i)}${stackPointer === i ? " < SP" : ""}`;
        // }
        for (let i = 0; i < memoryCellIds.length - 1; i++) {
            let cellId = memoryCellIds[i];
//           memoryCells[cellId].innerHTML = `${dec2bin(vm.peek(cellId))}${stackPointer === cellId ? " < SP" : ""}`;
             memoryCells[cellId].innerHTML = `${vm.peek(cellId)}${stackPointer === cellId ? " < SP" : ""}`;
        }
        memoryCells[24575].innerHTML = `${vm.peek(24575)} (key: ${String.fromCharCode(vm.peek(24575))})`;
    })();
});


