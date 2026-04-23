import type { Type1127 } from '../batch-05/file-1127.js';
export interface Type1128 {
  id: 1128;
  name: 'File1128';
  next: Type1127;
}

export function make1128(): Type1128 {
  return { id: 1128, name: 'File1128', next: null as unknown as Type1127 };
}
