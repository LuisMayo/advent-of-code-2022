import { get_text_input } from "../../lib.ts";

enum InstructionType {
    noop = "noop",
    addx = "addx"
}

class Instruction {
    opcode: InstructionType;
    operand?: number;

    constructor(ins: string) {
        const parts = ins.split(" ");
        this.opcode = parts[0] as InstructionType;
        if (parts[1]?.length > 0) {
            this.operand = +parts[1];
        }
    }
}

class CPU {
    program: Instruction[];
    pc = 0;
    x = 1;
    cycle = 0;
    cyclesLeftForInstruction = 0;
    pendingInstruction: Instruction | null = null;

    get halt() {
        return this.pc >= this.program.length;
    }

    constructor(program: string) {
        this.program = program.split("\n").map(line => new Instruction(line));
    }

    runCycle() {
        if (this.pendingInstruction) {
            this.cyclesLeftForInstruction--;
            if (this.cyclesLeftForInstruction <= 0) {
                // If the operation was left pending we can assume it was an addx
                this.x += this.pendingInstruction.operand!;
                this.pendingInstruction = null;
            }
        } else {
            const instruction = this.program[this.pc++];
            if (instruction.opcode === InstructionType.addx) {
                this.cyclesLeftForInstruction = 1;
                this.pendingInstruction = instruction;
            }
        }
        this.cycle++;
    }
}

function main() {
    const input = get_text_input("10", "full");
    const cpu = new CPU(input);
    let buffer = "";
    while (!cpu.halt) {
        const position = cpu.cycle % 40;
        if (position === 0) {
            buffer += "\n";
        }
        buffer += Math.abs(cpu.x - position) <= 1 ? '#' : '.';
        cpu.runCycle();
    }
    
    console.log(buffer);
}

main();
