import type { Type1 } from '../batch-00/file-0001.js';
export interface Type2 {
  id: 2;
  name: 'File2';
  next: Type1;
}

export function make2(): Type2 {
  return { id: 2, name: 'File2', next: null as unknown as Type1 };
}
