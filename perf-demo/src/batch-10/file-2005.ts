import type { Type2004 } from '../batch-10/file-2004.js';
export interface Type2005 {
  id: 2005;
  name: 'File2005';
  next: Type2004;
}

export function make2005(): Type2005 {
  return { id: 2005, name: 'File2005', next: null as unknown as Type2004 };
}
