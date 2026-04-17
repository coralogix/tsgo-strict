import type { Type2380 } from '../batch-11/file-2380.js';
export interface Type2381 {
  id: 2381;
  name: 'File2381';
  next: Type2380;
}

export function make2381(): Type2381 {
  return { id: 2381, name: 'File2381', next: null as unknown as Type2380 };
}
