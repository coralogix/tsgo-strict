import type { Type1940 } from '../batch-09/file-1940.js';
export interface Type1941 {
  id: 1941;
  name: 'File1941';
  next: Type1940;
}

export function make1941(): Type1941 {
  return { id: 1941, name: 'File1941', next: null as unknown as Type1940 };
}
