import type { Type1213 } from '../batch-06/file-1213.js';
export interface Type1214 {
  id: 1214;
  name: 'File1214';
  next: Type1213;
}

export function make1214(): Type1214 {
  return { id: 1214, name: 'File1214', next: null as unknown as Type1213 };
}
