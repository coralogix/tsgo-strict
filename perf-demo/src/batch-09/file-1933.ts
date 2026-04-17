import type { Type1932 } from '../batch-09/file-1932.js';
export interface Type1933 {
  id: 1933;
  name: 'File1933';
  next: Type1932;
}

export function make1933(): Type1933 {
  return { id: 1933, name: 'File1933', next: null as unknown as Type1932 };
}
