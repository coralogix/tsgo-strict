import type { Type1789 } from '../batch-08/file-1789.js';
export interface Type1790 {
  id: 1790;
  name: 'File1790';
  next: Type1789;
}

export function make1790(): Type1790 {
  return { id: 1790, name: 'File1790', next: null as unknown as Type1789 };
}
