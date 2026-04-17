import type { Type2031 } from '../batch-10/file-2031.js';
export interface Type2032 {
  id: 2032;
  name: 'File2032';
  next: Type2031;
}

export function make2032(): Type2032 {
  return { id: 2032, name: 'File2032', next: null as unknown as Type2031 };
}
