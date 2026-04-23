import type { Type2200 } from '../batch-11/file-2200.js';
export interface Type2201 {
  id: 2201;
  name: 'File2201';
  next: Type2200;
}

export function make2201(): Type2201 {
  return { id: 2201, name: 'File2201', next: null as unknown as Type2200 };
}
