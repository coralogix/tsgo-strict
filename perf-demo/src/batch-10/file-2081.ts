import type { Type2080 } from '../batch-10/file-2080.js';
export interface Type2081 {
  id: 2081;
  name: 'File2081';
  next: Type2080;
}

export function make2081(): Type2081 {
  return { id: 2081, name: 'File2081', next: null as unknown as Type2080 };
}
