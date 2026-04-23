import type { Type2032 } from '../batch-10/file-2032.js';
export interface Type2033 {
  id: 2033;
  name: 'File2033';
  next: Type2032;
}

export function make2033(): Type2033 {
  return { id: 2033, name: 'File2033', next: null as unknown as Type2032 };
}
