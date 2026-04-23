import type { Type3256 } from '../batch-16/file-3256.js';
export interface Type3257 {
  id: 3257;
  name: 'File3257';
  next: Type3256;
}

export function make3257(): Type3257 {
  return { id: 3257, name: 'File3257', next: null as unknown as Type3256 };
}
