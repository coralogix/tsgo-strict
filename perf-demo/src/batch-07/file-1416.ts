import type { Type1415 } from '../batch-07/file-1415.js';
export interface Type1416 {
  id: 1416;
  name: 'File1416';
  next: Type1415;
}

export function make1416(): Type1416 {
  return { id: 1416, name: 'File1416', next: null as unknown as Type1415 };
}
