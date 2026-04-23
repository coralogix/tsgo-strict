import type { Type2001 } from '../batch-10/file-2001.js';
export interface Type2002 {
  id: 2002;
  name: 'File2002';
  next: Type2001;
}

export function make2002(): Type2002 {
  return { id: 2002, name: 'File2002', next: null as unknown as Type2001 };
}
