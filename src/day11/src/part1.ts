import { get_text_input } from "../../lib.ts";

class Monkey {
    public heldItems: number[];
    private operation: (item: number) => number;
    module: number;
    ifTrue: number;
    ifFalse: number;
    counter = 0;
    constructor(str: string) {
        const lines = str.split("\n").slice(1);
        const firstLinePayload = lines[0].replace("Starting items: ", "");
        this.heldItems = firstLinePayload.split(", ").map((item) => +item);
        const operationPayload = lines[1].replace("Operation: new = ", "");
        this.operation = (item: number) => {
            const finalPayload = operationPayload.replaceAll("old", item.toString());
            return Math.floor(eval(finalPayload));
        };
        const testPayload = lines[2].replace("Test: divisible by ", "");
        const trueOutcomePayload = lines[3].replace("If true: throw to monkey ", "");
        const falseOutcomePayload = lines[4].replace("If false: throw to monkey ", "");
        this.module = +testPayload;
        this.ifTrue = +trueOutcomePayload;
        this.ifFalse = +falseOutcomePayload;
    }

    checkWhoToSend(idx: number) {
        const item = this.heldItems[idx];
        return item % this.module === 0 ? this.ifTrue : this.ifFalse;
    }

    operateOnIndex(idx: number) {
        this.heldItems[idx] = this.operation(this.heldItems[idx]);
        this.heldItems[idx] = Math.floor(this.heldItems[idx] / 3);
        this.counter++;
    }
}

function main() {
    const input = get_text_input("11", "full");
    const monkeyInputs = input.split("\n\n");
    const monkeys = monkeyInputs.map((input) => new Monkey(input));
    for (let i = 0; i < 20; i++) {
        for (const monkey of monkeys) {
            const destinies: number[] = [];
            monkey.heldItems.forEach((_item, idx) => {
                monkey.operateOnIndex(idx);
                const destiny = monkey.checkWhoToSend(idx);
                destinies.push(destiny);
            });
            destinies.forEach((destiny, idx) => {
                monkeys[destiny].heldItems.push(monkey.heldItems[idx]);
            });
            monkey.heldItems = [];
        }
    }
    monkeys.sort((a, b) => b.counter-a.counter);
    console.log(monkeys[0].counter * monkeys[1].counter);
}

main();