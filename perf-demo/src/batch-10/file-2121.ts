import type { Type2120 } from '../batch-10/file-2120.js';
export interface Type2121 {
  id: 2121;
  name: 'File2121';
  next: Type2120;
}

export function make2121(): Type2121 {
  return { id: 2121, name: 'File2121', next: null as unknown as Type2120 };
}
