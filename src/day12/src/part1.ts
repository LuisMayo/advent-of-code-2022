import { get_text_input } from "../../lib.ts";

export class AStarNode {
    public computedCost: number;
    expectedCost: number;
    get estimatedTotalCost() {
        return (this.computedCost || 0) + (this.expectedCost || 0);
    }

    constructor(public innerNode: MapNode, end: MapNode, public parent?: AStarNode) {
        this.computedCost = parent ? parent.computedCost + 1 : 0;
        const dRow = Math.abs(this.innerNode.rowIdx - end.rowIdx);
        const dColumn = Math.abs(this.innerNode.colIdx - end.colIdx);
        this.expectedCost = dRow * dColumn;
        // this.expectedCost = 1;
    }

    static aStarTimeBaby(start: MapNode, end: MapNode) {
        const openNodes: AStarNode[] = [new AStarNode(start, end)];
        const closedNodes: MapNode[] = [];
        let iterations = 0;
        while (openNodes.length > 0) {
            iterations++;
            // console.log(openNodes.length);
            const currentNode = openNodes.pop()!;
            if (currentNode.innerNode == end) {
                console.log(iterations);
                return currentNode;
            }
            closedNodes.push(currentNode.innerNode);
            for (const connection of currentNode.innerNode.connections) {
                if (!closedNodes.includes(connection)) {
                    const aStarConnection = new AStarNode(connection, end, currentNode);
                    const existingOpenNode = openNodes.findIndex(item => item.innerNode === connection);
                    if (existingOpenNode > -1) {
                        if (openNodes[existingOpenNode].computedCost > aStarConnection.computedCost) {
                            // console.log("Replacing worst path to node");
                            openNodes.splice(existingOpenNode, 1);
                            openNodes.push(aStarConnection);
                        } else {
                            // console.log(`There was a better path anyway, old ${openNodes[existingOpenNode].computedCost}, new ${aStarConnection.computedCost}`);
                        }
                    } else {
                        // console.log("Adding new unexplored path");
                        openNodes.push(aStarConnection);
                    }
                }
            }
            openNodes.sort((a, b) => b.estimatedTotalCost - a.estimatedTotalCost);
        }
    }
}

export class MapNode  {
    public connections: MapNode[] = [];
    public height!: number;
    rowIdx!: number;
    colIdx!: number;

    constructor() {}

    static computeConnections(map: MapNode[][]) {
        map.forEach((row, rowIdx) => {
            row.forEach((node, columnIdx) => {
                const adjacent = this.getAdjacents(map, rowIdx, columnIdx);
                node.connections = adjacent.filter(other => node.height >= other.height || other.height - node.height <= 1);
            });
        });
    }

    static getAdjacents(map: MapNode[][], rowIdx: number, columnIdx: number) {
        const adjacents: MapNode[] = [];
        if (rowIdx > 0) {
            adjacents.push(map[rowIdx - 1][columnIdx]);
        }
        if (rowIdx < map.length - 1) {
            adjacents.push(map[rowIdx + 1][columnIdx]);
        }
        if (columnIdx > 0) {
            adjacents.push(map[rowIdx][columnIdx - 1]);
        }
        if (columnIdx < map[0].length - 1) {
            adjacents.push(map[rowIdx][columnIdx + 1]);
        }
        return adjacents;
    }
}

function main() {
    const input = get_text_input("12", "full");
    // Row/column
    const map: MapNode[][] = [];
    let startNode: MapNode;
    let endNode: MapNode;
    let i = 0;
    for (const row of  input.split("\n")) {
        const currentRow: MapNode[] = [];
        map.push(currentRow);
        let j = 0;
        for (const char of row) {
            let height: number;
            const node = new MapNode();
            switch (char) {
                case "S":
                    height = "a".charCodeAt(0);
                    startNode = node;
                    break;
                case "E":
                    height = "z".charCodeAt(0);
                    endNode = node;
                    break;
                default:
                    height = char.charCodeAt(0);
                    break;
            }
            node.rowIdx = i;
            node.colIdx = j;
            node.height = height;
            currentRow.push(node);
            j++;
        }
        i++;
    }
    MapNode.computeConnections(map);
    const result = AStarNode.aStarTimeBaby(startNode!, endNode!);
    console.log("=====================");
    console.log(result?.computedCost);
}

main();
