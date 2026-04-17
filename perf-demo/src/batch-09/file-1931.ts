import type { Type1930 } from '../batch-09/file-1930.js';
export interface Type1931 {
  id: 1931;
  name: 'File1931';
  next: Type1930;
}

export function make1931(): Type1931 {
  return { id: 1931, name: 'File1931', next: null as unknown as Type1930 };
}
