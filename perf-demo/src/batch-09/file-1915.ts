import type { Type1914 } from '../batch-09/file-1914.js';
export interface Type1915 {
  id: 1915;
  name: 'File1915';
  next: Type1914;
}

export function make1915(): Type1915 {
  return { id: 1915, name: 'File1915', next: null as unknown as Type1914 };
}
