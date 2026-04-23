import type { Type2121 } from '../batch-10/file-2121.js';
export interface Type2122 {
  id: 2122;
  name: 'File2122';
  next: Type2121;
}

export function make2122(): Type2122 {
  return { id: 2122, name: 'File2122', next: null as unknown as Type2121 };
}
