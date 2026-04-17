import type { Type1130 } from '../batch-05/file-1130.js';
export interface Type1131 {
  id: 1131;
  name: 'File1131';
  next: Type1130;
}

export function make1131(): Type1131 {
  return { id: 1131, name: 'File1131', next: null as unknown as Type1130 };
}
