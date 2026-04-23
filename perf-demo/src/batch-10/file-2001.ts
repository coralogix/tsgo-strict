import type { Type2000 } from '../batch-10/file-2000.js';
export interface Type2001 {
  id: 2001;
  name: 'File2001';
  next: Type2000;
}

export function make2001(): Type2001 {
  return { id: 2001, name: 'File2001', next: null as unknown as Type2000 };
}
