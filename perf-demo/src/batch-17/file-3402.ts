import type { Type3401 } from '../batch-17/file-3401.js';
export interface Type3402 {
  id: 3402;
  name: 'File3402';
  next: Type3401;
}

export function make3402(): Type3402 {
  return { id: 3402, name: 'File3402', next: null as unknown as Type3401 };
}
