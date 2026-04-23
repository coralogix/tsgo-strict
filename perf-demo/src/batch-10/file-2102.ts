import type { Type2101 } from '../batch-10/file-2101.js';
export interface Type2102 {
  id: 2102;
  name: 'File2102';
  next: Type2101;
}

export function make2102(): Type2102 {
  return { id: 2102, name: 'File2102', next: null as unknown as Type2101 };
}
