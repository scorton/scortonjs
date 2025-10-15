declare module 'child_process' {
  export function spawn(cmd: any, args?: any[], options?: any): any;
  export function execSync(cmd: any, options?: any): any;
}

declare module 'url' {
  export function fileURLToPath(url: any): any;
}

declare module 'path' {
  export function dirname(p: any): any;
  export function join(...parts: any[]): any;
}

declare module 'fs' {
  export function existsSync(path: any): boolean;
  export function mkdirSync(path: any, options?: any): void;
  export function readFileSync(path: any, encoding?: any): string;
  export function writeFileSync(path: any, data: any): void;
  export function readdirSync(path: any): string[];
}

declare module 'os' {
  export function homedir(): string;
}

declare class Buffer {
  toString(): string;
}

declare const process: any;

