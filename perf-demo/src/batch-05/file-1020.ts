import type { Type1019 } from '../batch-05/file-1019.js';
export interface Type1020 {
  id: 1020;
  name: 'File1020';
  next: Type1019;
}

export function make1020(): Type1020 {
  return { id: 1020, name: 'File1020', next: null as unknown as Type1019 };
}
