import type { Type2210 } from '../batch-11/file-2210.js';
export interface Type2211 {
  id: 2211;
  name: 'File2211';
  next: Type2210;
}

export function make2211(): Type2211 {
  return { id: 2211, name: 'File2211', next: null as unknown as Type2210 };
}
