import type { Type1125 } from '../batch-05/file-1125.js';
export interface Type1126 {
  id: 1126;
  name: 'File1126';
  next: Type1125;
}

export function make1126(): Type1126 {
  return { id: 1126, name: 'File1126', next: null as unknown as Type1125 };
}
