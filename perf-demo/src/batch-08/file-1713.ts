import type { Type1712 } from '../batch-08/file-1712.js';
export interface Type1713 {
  id: 1713;
  name: 'File1713';
  next: Type1712;
}

export function make1713(): Type1713 {
  return { id: 1713, name: 'File1713', next: null as unknown as Type1712 };
}
