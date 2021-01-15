import { JackVirtualMachine } from "jackvm-wasm";

const vm = JackVirtualMachine.new();

let progEl = document.querySelector("#editor");
console.log(progEl.value.split('\n').map(s => s.trim()));

let runEl = document.querySelector("#run");

let memoryCells = {};
for (let i = 256; i < 262; i++) {
    memoryCells[i] = document.querySelector(`#mem-${i}`);
}

runEl.addEventListener("click", event => {
    const prog = progEl.value.split('\n').map(s => s.trim());
    // console.log("loading program [", prog.join("\n"), "]");
    let result = vm.load(prog.join("\n"));
    if (!result.succeeded) {
        console.log("errors ****", result.get_errors());
    }
    for (let i = 0; i < 150; i++) {
        vm.tick();
    }
    console.log("**** value from VM: " + vm.peek(256));
    let stackPointer = vm.peek(0);
    for (let i = 256; i < 262; i++) {
        memoryCells[i].innerHTML = `${vm.peek(i)}${stackPointer === i ? " < SP" : ""}`;
    }
});


