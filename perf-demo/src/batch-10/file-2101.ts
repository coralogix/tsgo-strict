import type { Type2100 } from '../batch-10/file-2100.js';
export interface Type2101 {
  id: 2101;
  name: 'File2101';
  next: Type2100;
}

export function make2101(): Type2101 {
  return { id: 2101, name: 'File2101', next: null as unknown as Type2100 };
}
