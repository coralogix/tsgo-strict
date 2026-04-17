import type { Type2110 } from '../batch-10/file-2110.js';
export interface Type2111 {
  id: 2111;
  name: 'File2111';
  next: Type2110;
}

export function make2111(): Type2111 {
  return { id: 2111, name: 'File2111', next: null as unknown as Type2110 };
}
