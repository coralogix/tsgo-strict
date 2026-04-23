import type { Type1090 } from '../batch-05/file-1090.js';
export interface Type1091 {
  id: 1091;
  name: 'File1091';
  next: Type1090;
}

export function make1091(): Type1091 {
  return { id: 1091, name: 'File1091', next: null as unknown as Type1090 };
}
