import type { Type1995 } from '../batch-09/file-1995.js';
export interface Type1996 {
  id: 1996;
  name: 'File1996';
  next: Type1995;
}

export function make1996(): Type1996 {
  return { id: 1996, name: 'File1996', next: null as unknown as Type1995 };
}
