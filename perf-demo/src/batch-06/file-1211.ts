import type { Type1210 } from '../batch-06/file-1210.js';
export interface Type1211 {
  id: 1211;
  name: 'File1211';
  next: Type1210;
}

export function make1211(): Type1211 {
  return { id: 1211, name: 'File1211', next: null as unknown as Type1210 };
}
