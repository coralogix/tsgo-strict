import type { Type2006 } from '../batch-10/file-2006.js';
export interface Type2007 {
  id: 2007;
  name: 'File2007';
  next: Type2006;
}

export function make2007(): Type2007 {
  return { id: 2007, name: 'File2007', next: null as unknown as Type2006 };
}
