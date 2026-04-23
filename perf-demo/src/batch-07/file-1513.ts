import type { Type1512 } from '../batch-07/file-1512.js';
export interface Type1513 {
  id: 1513;
  name: 'File1513';
  next: Type1512;
}

export function make1513(): Type1513 {
  return { id: 1513, name: 'File1513', next: null as unknown as Type1512 };
}
