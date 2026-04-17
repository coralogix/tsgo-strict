import type { Type1115 } from '../batch-05/file-1115.js';
export interface Type1116 {
  id: 1116;
  name: 'File1116';
  next: Type1115;
}

export function make1116(): Type1116 {
  return { id: 1116, name: 'File1116', next: null as unknown as Type1115 };
}
