import type { Type1999 } from '../batch-09/file-1999.js';
export interface Type2000 {
  id: 2000;
  name: 'File2000';
  next: Type1999;
}

export function make2000(): Type2000 {
  return { id: 2000, name: 'File2000', next: null as unknown as Type1999 };
}
