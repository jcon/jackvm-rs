import { JackVirtualMachine } from "jackvm-wasm";

const vm = JackVirtualMachine.new();
vm.load("push constant 3\npush constant 4\nadd");
vm.tick();
vm.tick();
vm.tick();
console.log("**** value from VM: " + vm.peek(256));

let progEl = document.querySelector("#editor");
console.log(progEl.value.split('\n').map(s => s.trim()));