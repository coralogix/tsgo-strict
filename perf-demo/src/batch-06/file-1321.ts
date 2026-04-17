import type { Type1320 } from '../batch-06/file-1320.js';
export interface Type1321 {
  id: 1321;
  name: 'File1321';
  next: Type1320;
}

export function make1321(): Type1321 {
  return { id: 1321, name: 'File1321', next: null as unknown as Type1320 };
}
