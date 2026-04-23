import type { Type1939 } from '../batch-09/file-1939.js';
export interface Type1940 {
  id: 1940;
  name: 'File1940';
  next: Type1939;
}

export function make1940(): Type1940 {
  return { id: 1940, name: 'File1940', next: null as unknown as Type1939 };
}
