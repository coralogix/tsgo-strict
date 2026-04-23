import type { Type1950 } from '../batch-09/file-1950.js';
export interface Type1951 {
  id: 1951;
  name: 'File1951';
  next: Type1950;
}

export function make1951(): Type1951 {
  return { id: 1951, name: 'File1951', next: null as unknown as Type1950 };
}
