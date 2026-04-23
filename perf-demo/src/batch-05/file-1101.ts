import type { Type1100 } from '../batch-05/file-1100.js';
export interface Type1101 {
  id: 1101;
  name: 'File1101';
  next: Type1100;
}

export function make1101(): Type1101 {
  return { id: 1101, name: 'File1101', next: null as unknown as Type1100 };
}
