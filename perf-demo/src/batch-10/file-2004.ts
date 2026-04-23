import type { Type2003 } from '../batch-10/file-2003.js';
export interface Type2004 {
  id: 2004;
  name: 'File2004';
  next: Type2003;
}

export function make2004(): Type2004 {
  return { id: 2004, name: 'File2004', next: null as unknown as Type2003 };
}
