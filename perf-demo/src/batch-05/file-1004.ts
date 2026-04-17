import type { Type1003 } from '../batch-05/file-1003.js';
export interface Type1004 {
  id: 1004;
  name: 'File1004';
  next: Type1003;
}

export function make1004(): Type1004 {
  return { id: 1004, name: 'File1004', next: null as unknown as Type1003 };
}
