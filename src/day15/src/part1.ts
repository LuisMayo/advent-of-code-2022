import { get_text_input } from "../../lib.ts";

class Point {
    constructor(public x: number, public y: number) {}
    equals(other: Point)  {
        return this.x === other.x && this.y === other.y;
    }

    distanceTo(other: Point) {
        return Math.abs(this.x - other.x) + Math.abs(this.y - other.y);
    }
    toString() {
        return `(${this.x}, ${this.y})`;
    }
}

class SensorArea {
    private distanceToBeacon: number;
    constructor(public sensor: Point, public beacon: Point) {
        this.distanceToBeacon = sensor.distanceTo(beacon);
    }

    isPointClear(point: Point) {
        return point.distanceTo(this.sensor) <= this.distanceToBeacon;
    }

    maxCheckableX() {
        return this.sensor.x + this.distanceToBeacon;
    }
    minCheckableX() {
        return this.sensor.x - this.distanceToBeacon;
    }

    toString() {
        return this.sensor.toString();
    }
}

class BeaconMap {
    areas: SensorArea[] = [];
    beacons: Point[] = [];
    minCheckableX = Number.POSITIVE_INFINITY;
    maxCheckableX = Number.NEGATIVE_INFINITY;

    constructor(rawStr: string) {
        const regex = /Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)/;
        const lines = rawStr.split("\n");
        for (const line of lines) {
            const matches = (line.match(regex))!;
            const sensor = new Point(+matches[1], +matches[2]);
            const beacon = new Point(+matches[3], +matches[4]);
            this.beacons.push(beacon);
            const sensorArea = new SensorArea(sensor, beacon);
            if (sensorArea.maxCheckableX() > this.maxCheckableX) {
                this.maxCheckableX = sensorArea.maxCheckableX();
            }
            if (sensorArea.minCheckableX() < this.minCheckableX) {
                this.minCheckableX = sensorArea.minCheckableX();
            }
            this.areas.push(sensorArea);
        }
    }

    countImpossibleSpots(y: number) {
        let accum = 0;
        for (let x = this.minCheckableX; x <= this.maxCheckableX; x++) {
            const point = new Point(x, y);
            if (!this.beacons.find(beacon => beacon.equals(point))) {
                for (const area of this.areas) {
                    if (area.isPointClear(point)) {
                        // console.log(`Point ${point} can't have a bacon due to the influence of sensor ${area}`);
                        accum++;
                        break;
                    }
                    // console.log(`Point ${point} doesn't have a problem with area ${area}`);
                }
            }
        }
        return accum;
    }
}

function main() {
    console.time();
    const input = get_text_input("15", "full");
    const map = new BeaconMap(input);
    // console.log(map.countImpossibleSpots(10));
    console.log(map.countImpossibleSpots(2000000));
    console.timeEnd();
}

main();