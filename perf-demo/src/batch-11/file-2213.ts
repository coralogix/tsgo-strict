import type { Type2212 } from '../batch-11/file-2212.js';
export interface Type2213 {
  id: 2213;
  name: 'File2213';
  next: Type2212;
}

export function make2213(): Type2213 {
  return { id: 2213, name: 'File2213', next: null as unknown as Type2212 };
}
