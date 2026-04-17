import type { Type2303 } from '../batch-11/file-2303.js';
export interface Type2304 {
  id: 2304;
  name: 'File2304';
  next: Type2303;
}

export function make2304(): Type2304 {
  return { id: 2304, name: 'File2304', next: null as unknown as Type2303 };
}
