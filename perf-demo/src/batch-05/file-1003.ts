import type { Type1002 } from '../batch-05/file-1002.js';
export interface Type1003 {
  id: 1003;
  name: 'File1003';
  next: Type1002;
}

export function make1003(): Type1003 {
  return { id: 1003, name: 'File1003', next: null as unknown as Type1002 };
}
