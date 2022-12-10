export function get_text_input(day: string, part: string): string {
    return Deno.readTextFileSync(`./src/day${day}/input/${part}.txt`).replaceAll("\r", "");
}
