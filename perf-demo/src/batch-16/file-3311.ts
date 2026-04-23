import type { Type3310 } from '../batch-16/file-3310.js';
export interface Type3311 {
  id: 3311;
  name: 'File3311';
  next: Type3310;
}

export function make3311(): Type3311 {
  return { id: 3311, name: 'File3311', next: null as unknown as Type3310 };
}
