import type { Type1811 } from '../batch-09/file-1811.js';
export interface Type1812 {
  id: 1812;
  name: 'File1812';
  next: Type1811;
}

export function make1812(): Type1812 {
  return { id: 1812, name: 'File1812', next: null as unknown as Type1811 };
}
