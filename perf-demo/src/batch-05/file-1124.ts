import type { Type1123 } from '../batch-05/file-1123.js';
export interface Type1124 {
  id: 1124;
  name: 'File1124';
  next: Type1123;
}

export function make1124(): Type1124 {
  return { id: 1124, name: 'File1124', next: null as unknown as Type1123 };
}
