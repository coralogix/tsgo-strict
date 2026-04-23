import type { Type3002 } from '../batch-15/file-3002.js';
export interface Type3003 {
  id: 3003;
  name: 'File3003';
  next: Type3002;
}

export function make3003(): Type3003 {
  return { id: 3003, name: 'File3003', next: null as unknown as Type3002 };
}
