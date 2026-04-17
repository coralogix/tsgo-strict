import type { Type2048 } from '../batch-10/file-2048.js';
export interface Type2049 {
  id: 2049;
  name: 'File2049';
  next: Type2048;
}

export function make2049(): Type2049 {
  return { id: 2049, name: 'File2049', next: null as unknown as Type2048 };
}
