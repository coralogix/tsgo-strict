import type { Type2005 } from '../batch-10/file-2005.js';
export interface Type2006 {
  id: 2006;
  name: 'File2006';
  next: Type2005;
}

export function make2006(): Type2006 {
  return { id: 2006, name: 'File2006', next: null as unknown as Type2005 };
}
