import type { Type3210 } from '../batch-16/file-3210.js';
export interface Type3211 {
  id: 3211;
  name: 'File3211';
  next: Type3210;
}

export function make3211(): Type3211 {
  return { id: 3211, name: 'File3211', next: null as unknown as Type3210 };
}
