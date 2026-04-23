import type { Type1521 } from '../batch-07/file-1521.js';
export interface Type1522 {
  id: 1522;
  name: 'File1522';
  next: Type1521;
}

export function make1522(): Type1522 {
  return { id: 1522, name: 'File1522', next: null as unknown as Type1521 };
}
