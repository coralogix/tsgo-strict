import type { Type2300 } from '../batch-11/file-2300.js';
export interface Type2301 {
  id: 2301;
  name: 'File2301';
  next: Type2300;
}

export function make2301(): Type2301 {
  return { id: 2301, name: 'File2301', next: null as unknown as Type2300 };
}
