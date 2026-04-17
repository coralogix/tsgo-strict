import type { Type1012 } from '../batch-05/file-1012.js';
export interface Type1013 {
  id: 1013;
  name: 'File1013';
  next: Type1012;
}

export function make1013(): Type1013 {
  return { id: 1013, name: 'File1013', next: null as unknown as Type1012 };
}
