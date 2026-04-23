import type { Type2047 } from '../batch-10/file-2047.js';
export interface Type2048 {
  id: 2048;
  name: 'File2048';
  next: Type2047;
}

export function make2048(): Type2048 {
  return { id: 2048, name: 'File2048', next: null as unknown as Type2047 };
}
