import type { Type1004 } from '../batch-05/file-1004.js';
export interface Type1005 {
  id: 1005;
  name: 'File1005';
  next: Type1004;
}

export function make1005(): Type1005 {
  return { id: 1005, name: 'File1005', next: null as unknown as Type1004 };
}
