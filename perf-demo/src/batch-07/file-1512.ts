import type { Type1511 } from '../batch-07/file-1511.js';
export interface Type1512 {
  id: 1512;
  name: 'File1512';
  next: Type1511;
}

export function make1512(): Type1512 {
  return { id: 1512, name: 'File1512', next: null as unknown as Type1511 };
}
