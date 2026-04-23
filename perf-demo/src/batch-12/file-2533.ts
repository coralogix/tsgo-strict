import type { Type2532 } from '../batch-12/file-2532.js';
export interface Type2533 {
  id: 2533;
  name: 'File2533';
  next: Type2532;
}

export function make2533(): Type2533 {
  return { id: 2533, name: 'File2533', next: null as unknown as Type2532 };
}
