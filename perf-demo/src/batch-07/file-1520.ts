import type { Type1519 } from '../batch-07/file-1519.js';
export interface Type1520 {
  id: 1520;
  name: 'File1520';
  next: Type1519;
}

export function make1520(): Type1520 {
  return { id: 1520, name: 'File1520', next: null as unknown as Type1519 };
}
