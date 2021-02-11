// The virtual registers of the Hack computer.
const REGISTER_NAMES = {
    0: 'SP',
    1: 'ARG',
    2: 'LCL',
    3: 'THIS',
    4: 'THAT',
}
for (let i = 0; i < 8; i++) {
    REGISTER_NAMES[i + 5] = `TEMP${i}`;
}
for (let i = 13; i < 16; i++) {
    REGISTER_NAMES[i] = `R${i}`;
}

function range(start, end) {
    return [...Array(end - start).keys()].map((key) => key + start);
}

function createMemoryCell(address) {
    let tableRow = document.createElement("tr");
    let rowHead = document.createElement("th");
    const name = address in REGISTER_NAMES ? REGISTER_NAMES[address] : address.toString();
    rowHead.innerHTML = name;
    tableRow.appendChild(rowHead);
    let rowCell = document.createElement("td");
    rowCell.innerHTML = "0";
    tableRow.appendChild(rowCell);
    return [tableRow, rowCell];
}

class MemoryDebugger {
    constructor(vm) {
        this.vm = vm;
        let registerCellIds = range(0, 5);
        let memoryCellIds = range(256, 268); // [256, 257, 258, 259, 260, 261, 262, 24576]; // , 16384, 16416, 16448];

        this.memoryCells = {};
        this.initializedMemory = false;
        this.allMemoryCellIds = [];
        this.createCells('#memory-body', memoryCellIds);
        this.createCells('#registers-body', registerCellIds);
        this.allMemoryCellIds = registerCellIds.concat(memoryCellIds);
        this.initializedMemory = true;
    }

    createCells(tableId, cellIds) {
        let tableBody = document.querySelector(tableId);
        if (tableBody !== undefined) {
            for (let i = 0; i < cellIds.length; i++) {
                let [tableRow, rowCell] = createMemoryCell(cellIds[i]);
                tableBody.appendChild(tableRow);
                let cellId = cellIds[i];

                this.memoryCells[cellId] = rowCell;
            }
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
    }

}

export {
    MemoryDebugger
};