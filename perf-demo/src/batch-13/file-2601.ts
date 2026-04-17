import type { Type2600 } from '../batch-13/file-2600.js';
export interface Type2601 {
  id: 2601;
  name: 'File2601';
  next: Type2600;
}

export function make2601(): Type2601 {
  return { id: 2601, name: 'File2601', next: null as unknown as Type2600 };
}
