import type { Type1029 } from '../batch-05/file-1029.js';
export interface Type1030 {
  id: 1030;
  name: 'File1030';
  next: Type1029;
}

export function make1030(): Type1030 {
  return { id: 1030, name: 'File1030', next: null as unknown as Type1029 };
}
