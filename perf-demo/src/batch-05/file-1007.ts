import type { Type1006 } from '../batch-05/file-1006.js';
export interface Type1007 {
  id: 1007;
  name: 'File1007';
  next: Type1006;
}

export function make1007(): Type1007 {
  return { id: 1007, name: 'File1007', next: null as unknown as Type1006 };
}
