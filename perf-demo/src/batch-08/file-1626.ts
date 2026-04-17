import type { Type1625 } from '../batch-08/file-1625.js';
export interface Type1626 {
  id: 1626;
  name: 'File1626';
  next: Type1625;
}

export function make1626(): Type1626 {
  return { id: 1626, name: 'File1626', next: null as unknown as Type1625 };
}
