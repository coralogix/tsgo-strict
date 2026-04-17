import type { Type2049 } from '../batch-10/file-2049.js';
export interface Type2050 {
  id: 2050;
  name: 'File2050';
  next: Type2049;
}

export function make2050(): Type2050 {
  return { id: 2050, name: 'File2050', next: null as unknown as Type2049 };
}
