import type { Type1800 } from '../batch-09/file-1800.js';
export interface Type1801 {
  id: 1801;
  name: 'File1801';
  next: Type1800;
}

export function make1801(): Type1801 {
  return { id: 1801, name: 'File1801', next: null as unknown as Type1800 };
}
