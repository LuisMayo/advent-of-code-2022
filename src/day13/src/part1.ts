import { get_text_input } from "../../lib.ts";

type Item = number | Item[];

function compareItem(a: Item, b: Item): boolean | null {
    const typeA = typeof a;
    const typeB = typeof b;
    if (typeA ===typeB) {
        if (typeA === "number") {
            if (a === b) {
                return null;
            } else {
                return a < b;
            }
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

function compareLists(a: Item[], b: Item[]): boolean | null {
    let check: null | boolean = null;
    for (let i = 0; check == null && i < a.length && i < b.length; i++) {
        check = compareItem(a[i], b[i]);
    }

    if (check == null) {
        if (a.length !== b.length) {
            check = a.length < b.length;
        }
    }
    return check;
}

function parsePacket(rawStr: string): Item[] {
    return eval(rawStr);
    // const str = rawStr.substring(1, rawStr.length - 1);
    // const packet: Item = [];
    // let depth = 0;
    // let currentNumberBuffer = "";
    // // Buffering all itmes if we're parsing a list
    // let buffer = "";
    // for (const char of str) {
    //     if (/\d/.test(char) && depth === 0) {
    //         currentNumberBuffer += char;
    //     } else {
    //         if (currentNumberBuffer.length > 0) {
    //             packet.push(+currentNumberBuffer);
    //         }
    //         currentNumberBuffer = "";
    //         buffer += char;
    //         if(char === "[") {
    //             depth++;
    //         }   
    //         if(char === "]") {
    //             depth--;
    //             if (depth === 0) {
    //                 packet.push(parsePacket(buffer));
    //             }
    //         }
    //     }
    //     if (depth === 0) {
    //         buffer = "";
    //     }
    // }
    // if (currentNumberBuffer.length > 0) {
    //     packet.push(+currentNumberBuffer);
    // }
    // return [packet];
}

function main() {
    const input = get_text_input("13", "full");
    const pairs = input.split("\n\n");
    let accum = 0;
    pairs.forEach((pair, idx) => {
        const split = pair.split("\n");
        const a = parsePacket(split[0]);
        const b = parsePacket(split[1]);
        if (compareLists(a, b)) {
            accum += idx + 1;
        }
    });

    console.log(accum);
}

main();
