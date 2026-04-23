import type { Type1234 } from '../batch-06/file-1234.js';
export interface Type1235 {
  id: 1235;
  name: 'File1235';
  next: Type1234;
}

export function make1235(): Type1235 {
  return { id: 1235, name: 'File1235', next: null as unknown as Type1234 };
}
