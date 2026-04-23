import type { Type2280 } from '../batch-11/file-2280.js';
export interface Type2281 {
  id: 2281;
  name: 'File2281';
  next: Type2280;
}

export function make2281(): Type2281 {
  return { id: 2281, name: 'File2281', next: null as unknown as Type2280 };
}
