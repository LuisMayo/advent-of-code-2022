import { get_text_input } from "../../lib.ts";

type Item = number | Item[];

function compareItem(a: Item, b: Item): number {
    const typeA = typeof a;
    const typeB = typeof b;
    if (typeA ===typeB) {
        if (typeA === "number") {
            return a as number - (b as number);
        } else {
            return compareLists(a as Item[], b as Item[]);
        }
    } else {
        if (typeA === "number") {
            a = [a];
        } else {
            b = [b];
        }
        return compareLists(a as Item[], b as Item[]);
    }
}

function compareLists(a: Item[], b: Item[]): number {
    let check = 0;
    for (let i = 0; check == 0 && i < a.length && i < b.length; i++) {
        check = compareItem(a[i], b[i]);
    }

    if (check == 0) {
        if (a.length !== b.length) {
            check = a.length - b.length;
        }
    }
    return check;
}

function parsePacket(rawStr: string): Item[] {
    return eval(rawStr);
}

function main() {
    const input = get_text_input("13", "full");
    const lines = input.split("\n").filter(item => item.trim().length > 0);
    const packets = lines.map(line => parsePacket(line));
    const markers = [[[2]], [[6]]];
    packets.push(...markers);
    packets.sort(compareLists);
    let accum = 1;
    for (const marker of markers) {
        const idx = packets.findIndex(item => item === marker);
        accum *= idx + 1;
    }
    // console.log(packets.join("\n"));

    console.log(accum);
}

main();
