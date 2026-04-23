import type { Type1994 } from '../batch-09/file-1994.js';
export interface Type1995 {
  id: 1995;
  name: 'File1995';
  next: Type1994;
}

export function make1995(): Type1995 {
  return { id: 1995, name: 'File1995', next: null as unknown as Type1994 };
}
