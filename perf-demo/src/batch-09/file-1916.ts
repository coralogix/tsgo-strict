import type { Type1915 } from '../batch-09/file-1915.js';
export interface Type1916 {
  id: 1916;
  name: 'File1916';
  next: Type1915;
}

export function make1916(): Type1916 {
  return { id: 1916, name: 'File1916', next: null as unknown as Type1915 };
}
