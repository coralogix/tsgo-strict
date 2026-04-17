import type { Type2040 } from '../batch-10/file-2040.js';
export interface Type2041 {
  id: 2041;
  name: 'File2041';
  next: Type2040;
}

export function make2041(): Type2041 {
  return { id: 2041, name: 'File2041', next: null as unknown as Type2040 };
}
