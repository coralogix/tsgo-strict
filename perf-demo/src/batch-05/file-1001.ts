import type { Type1000 } from '../batch-05/file-1000.js';
export interface Type1001 {
  id: 1001;
  name: 'File1001';
  next: Type1000;
}

export function make1001(): Type1001 {
  return { id: 1001, name: 'File1001', next: null as unknown as Type1000 };
}
