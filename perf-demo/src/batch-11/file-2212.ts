import type { Type2211 } from '../batch-11/file-2211.js';
export interface Type2212 {
  id: 2212;
  name: 'File2212';
  next: Type2211;
}

export function make2212(): Type2212 {
  return { id: 2212, name: 'File2212', next: null as unknown as Type2211 };
}
