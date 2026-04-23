import type { Type2255 } from '../batch-11/file-2255.js';
export interface Type2256 {
  id: 2256;
  name: 'File2256';
  next: Type2255;
}

export function make2256(): Type2256 {
  return { id: 2256, name: 'File2256', next: null as unknown as Type2255 };
}
