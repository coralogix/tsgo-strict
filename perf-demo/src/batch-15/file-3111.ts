import type { Type3110 } from '../batch-15/file-3110.js';
export interface Type3111 {
  id: 3111;
  name: 'File3111';
  next: Type3110;
}

export function make3111(): Type3111 {
  return { id: 3111, name: 'File3111', next: null as unknown as Type3110 };
}
