import type { Type3140 } from '../batch-15/file-3140.js';
export interface Type3141 {
  id: 3141;
  name: 'File3141';
  next: Type3140;
}

export function make3141(): Type3141 {
  return { id: 3141, name: 'File3141', next: null as unknown as Type3140 };
}
