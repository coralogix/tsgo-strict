import type { Type2122 } from '../batch-10/file-2122.js';
export interface Type2123 {
  id: 2123;
  name: 'File2123';
  next: Type2122;
}

export function make2123(): Type2123 {
  return { id: 2123, name: 'File2123', next: null as unknown as Type2122 };
}
