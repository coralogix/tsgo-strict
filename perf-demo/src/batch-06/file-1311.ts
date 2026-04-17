import type { Type1310 } from '../batch-06/file-1310.js';
export interface Type1311 {
  id: 1311;
  name: 'File1311';
  next: Type1310;
}

export function make1311(): Type1311 {
  return { id: 1311, name: 'File1311', next: null as unknown as Type1310 };
}
