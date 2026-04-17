import type { Type1221 } from '../batch-06/file-1221.js';
export interface Type1222 {
  id: 1222;
  name: 'File1222';
  next: Type1221;
}

export function make1222(): Type1222 {
  return { id: 1222, name: 'File1222', next: null as unknown as Type1221 };
}
