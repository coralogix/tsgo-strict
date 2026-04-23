import type { Type1340 } from '../batch-06/file-1340.js';
export interface Type1341 {
  id: 1341;
  name: 'File1341';
  next: Type1340;
}

export function make1341(): Type1341 {
  return { id: 1341, name: 'File1341', next: null as unknown as Type1340 };
}
