import type { Type1124 } from '../batch-05/file-1124.js';
export interface Type1125 {
  id: 1125;
  name: 'File1125';
  next: Type1124;
}

export function make1125(): Type1125 {
  return { id: 1125, name: 'File1125', next: null as unknown as Type1124 };
}
