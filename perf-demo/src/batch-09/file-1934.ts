import type { Type1933 } from '../batch-09/file-1933.js';
export interface Type1934 {
  id: 1934;
  name: 'File1934';
  next: Type1933;
}

export function make1934(): Type1934 {
  return { id: 1934, name: 'File1934', next: null as unknown as Type1933 };
}
