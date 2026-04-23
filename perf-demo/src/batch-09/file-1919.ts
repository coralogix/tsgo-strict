import type { Type1918 } from '../batch-09/file-1918.js';
export interface Type1919 {
  id: 1919;
  name: 'File1919';
  next: Type1918;
}

export function make1919(): Type1919 {
  return { id: 1919, name: 'File1919', next: null as unknown as Type1918 };
}
