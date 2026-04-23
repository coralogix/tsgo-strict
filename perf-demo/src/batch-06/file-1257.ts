import type { Type1256 } from '../batch-06/file-1256.js';
export interface Type1257 {
  id: 1257;
  name: 'File1257';
  next: Type1256;
}

export function make1257(): Type1257 {
  return { id: 1257, name: 'File1257', next: null as unknown as Type1256 };
}
