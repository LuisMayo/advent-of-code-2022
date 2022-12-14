import { get_text_input } from "../../lib.ts";

enum Tile {AIR, ROCK, SAND}

class Point {
    constructor(public x: number, public y: number) {}
    equals(other: Point)  {
        return this.x === other.x && this.y === other.y;
    }

    downPoint() {
        return new Point(this.x, this.y + 1);
    }

    downLeftPoint() {
        return new Point(this.x - 1, this.y + 1);
    }

    downRightPoint() {
        return new Point(this.x + 1, this.y + 1);
    }

    toString() {
        return `(${this.x}, ${this.y})`;
    }
}

class RockFormation {
    private turnPoints: Point[] = [];
    constructor(line: string) {
        this.turnPoints = line.split("->").map(coords => {
            const split = coords.split(",");
            return new Point(+split[0], +split[1]);
        });
    }

    getAllPointsInFormation() {
        const allPoints = [...this.turnPoints]
        for (let i = 1; i < this.turnPoints.length; i++) {
            const xDiff = this.turnPoints[i - 1].x - this.turnPoints[i].x;
            const yDiff = this.turnPoints[i - 1].y - this.turnPoints[i].y;
            for (let j = 1; j < Math.abs(xDiff); j++) {
                const x = this.turnPoints[i].x + Math.sign(xDiff) * j;
                const newPoint = new Point(x, this.turnPoints[i].y)
                allPoints.push(newPoint);
            }
            for (let j = 1; j < Math.abs(yDiff); j++) {
                const y = this.turnPoints[i].y + Math.sign(yDiff) * j;
                const newPoint = new Point(this.turnPoints[i].x, y);
                allPoints.push(newPoint);
            }
        }
        return allPoints;
    }
}

class MapSimulation {
    private tiles: Tile[][] = [];
    readonly sandGenerator = new Point(500, 0);
    private simulatedMovingSandPosition: Point | null = null;
    voidY: number;
    tick = 0;
    constructor(rawStr: string) {
        const rockFormations = rawStr.split("\n").map(line => new RockFormation(line));
        const allRockPoints = rockFormations.map(formation => formation.getAllPointsInFormation()).flat();
        this.voidY = Math.max(...allRockPoints.map(point => point.y));
        console.log(this.voidY);
        allRockPoints.forEach(point => this.safeTileSet(point, Tile.ROCK));
    }

    /**
     * 
     * @returns true if the simulation can still run
     */
    runTick(): boolean {
        this.tick++;
        if (this.simulatedMovingSandPosition == null) {
            if (this.isTileFree(this.sandGenerator)) {
                console.log("Generating new sand");
                this.safeTileSet(this.sandGenerator, Tile.SAND);
                this.simulatedMovingSandPosition = this.sandGenerator;
                return true;
            } else {
                return false;
            }

        } else if (this.simulatedMovingSandPosition.y > this.voidY) {
            this.safeTileSet(this.simulatedMovingSandPosition, Tile.AIR);
            return false;
        } else {
            const pointsToCheck = [this.simulatedMovingSandPosition.downPoint(), this.simulatedMovingSandPosition.downLeftPoint(), this.simulatedMovingSandPosition.downRightPoint()];
            const foundPoint = pointsToCheck.find(point => this.isTileFree(point));
            if (foundPoint) {
                console.log(`Moving Sand from ${this.simulatedMovingSandPosition} to ${foundPoint}`);
                this.safeTileSet(this.simulatedMovingSandPosition, Tile.AIR);
                this.safeTileSet(foundPoint, Tile.SAND);
                this.simulatedMovingSandPosition = foundPoint;
            } else {
                console.log(`Stoping sand at ${this.simulatedMovingSandPosition}`);
                this.simulatedMovingSandPosition = null;
            }
            return true;
        }
    }

    getSandTilesNumber() {
        return this.tiles.flat().filter(tile => tile === Tile.SAND).length;
    }

    private isTileFree(point: Point) {
        return this.safeTileGet(point) === Tile.AIR;
    }

    private safeTileGet(point: Point): Tile {
        this.ensureTileExists(point);
        return this.tiles[point.x][point.y];
    }

    private safeTileSet(point: Point, val: Tile) {
        this.ensureTileExists(point);
        this.tiles[point.x][point.y] = val;
    }

   private ensureTileExists(point: Point) {
    if(this.tiles[point.x] == null) {
      this.tiles[point.x] = [];
    }
    if(this.tiles[point.x][point.y] == null) {
      this.tiles[point.x][point.y] = Tile.AIR;
    }
  }
}

function main() {
    const input = get_text_input("14", "full");
    const simulation = new MapSimulation(input);
    while(simulation.runTick());
    console.log(simulation.getSandTilesNumber());
}

main();

