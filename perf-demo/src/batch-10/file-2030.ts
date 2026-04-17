import type { Type2029 } from '../batch-10/file-2029.js';
export interface Type2030 {
  id: 2030;
  name: 'File2030';
  next: Type2029;
}

export function make2030(): Type2030 {
  return { id: 2030, name: 'File2030', next: null as unknown as Type2029 };
}
