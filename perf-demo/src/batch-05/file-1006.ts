import type { Type1005 } from '../batch-05/file-1005.js';
export interface Type1006 {
  id: 1006;
  name: 'File1006';
  next: Type1005;
}

export function make1006(): Type1006 {
  return { id: 1006, name: 'File1006', next: null as unknown as Type1005 };
}
