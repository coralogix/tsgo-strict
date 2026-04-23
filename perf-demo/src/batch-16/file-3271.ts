import type { Type3270 } from '../batch-16/file-3270.js';
export interface Type3271 {
  id: 3271;
  name: 'File3271';
  next: Type3270;
}

export function make3271(): Type3271 {
  return { id: 3271, name: 'File3271', next: null as unknown as Type3270 };
}
