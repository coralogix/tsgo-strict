import type { Type1780 } from '../batch-08/file-1780.js';
export interface Type1781 {
  id: 1781;
  name: 'File1781';
  next: Type1780;
}

export function make1781(): Type1781 {
  return { id: 1781, name: 'File1781', next: null as unknown as Type1780 };
}
