import type { Type1998 } from '../batch-09/file-1998.js';
export interface Type1999 {
  id: 1999;
  name: 'File1999';
  next: Type1998;
}

export function make1999(): Type1999 {
  return { id: 1999, name: 'File1999', next: null as unknown as Type1998 };
}
