import type { Type2160 } from '../batch-10/file-2160.js';
export interface Type2161 {
  id: 2161;
  name: 'File2161';
  next: Type2160;
}

export function make2161(): Type2161 {
  return { id: 2161, name: 'File2161', next: null as unknown as Type2160 };
}
