import type { Type1001 } from '../batch-05/file-1001.js';
export interface Type1002 {
  id: 1002;
  name: 'File1002';
  next: Type1001;
}

export function make1002(): Type1002 {
  return { id: 1002, name: 'File1002', next: null as unknown as Type1001 };
}
