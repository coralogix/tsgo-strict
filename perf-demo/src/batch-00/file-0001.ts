import type { Type0 } from '../batch-00/file-0000.js';
export interface Type1 {
  id: 1;
  name: 'File1';
  next: Type0;
}

export function make1(): Type1 {
  return { id: 1, name: 'File1', next: null as unknown as Type0 };
}
