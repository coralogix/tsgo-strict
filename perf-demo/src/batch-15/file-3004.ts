import type { Type3003 } from '../batch-15/file-3003.js';
export interface Type3004 {
  id: 3004;
  name: 'File3004';
  next: Type3003;
}

export function make3004(): Type3004 {
  return { id: 3004, name: 'File3004', next: null as unknown as Type3003 };
}
