import type { Type1160 } from '../batch-05/file-1160.js';
export interface Type1161 {
  id: 1161;
  name: 'File1161';
  next: Type1160;
}

export function make1161(): Type1161 {
  return { id: 1161, name: 'File1161', next: null as unknown as Type1160 };
}
