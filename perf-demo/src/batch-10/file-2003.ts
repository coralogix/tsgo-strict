import type { Type2002 } from '../batch-10/file-2002.js';
export interface Type2003 {
  id: 2003;
  name: 'File2003';
  next: Type2002;
}

export function make2003(): Type2003 {
  return { id: 2003, name: 'File2003', next: null as unknown as Type2002 };
}
