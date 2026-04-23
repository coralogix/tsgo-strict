import type { Type1531 } from '../batch-07/file-1531.js';
export interface Type1532 {
  id: 1532;
  name: 'File1532';
  next: Type1531;
}

export function make1532(): Type1532 {
  return { id: 1532, name: 'File1532', next: null as unknown as Type1531 };
}
