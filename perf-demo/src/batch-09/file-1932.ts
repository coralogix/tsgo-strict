import type { Type1931 } from '../batch-09/file-1931.js';
export interface Type1932 {
  id: 1932;
  name: 'File1932';
  next: Type1931;
}

export function make1932(): Type1932 {
  return { id: 1932, name: 'File1932', next: null as unknown as Type1931 };
}
