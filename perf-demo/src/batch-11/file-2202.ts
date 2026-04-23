import type { Type2201 } from '../batch-11/file-2201.js';
export interface Type2202 {
  id: 2202;
  name: 'File2202';
  next: Type2201;
}

export function make2202(): Type2202 {
  return { id: 2202, name: 'File2202', next: null as unknown as Type2201 };
}
