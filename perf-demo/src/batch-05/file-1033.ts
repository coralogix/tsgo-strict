import type { Type1032 } from '../batch-05/file-1032.js';
export interface Type1033 {
  id: 1033;
  name: 'File1033';
  next: Type1032;
}

export function make1033(): Type1033 {
  return { id: 1033, name: 'File1033', next: null as unknown as Type1032 };
}
