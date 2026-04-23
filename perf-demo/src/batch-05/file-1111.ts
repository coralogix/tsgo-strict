import type { Type1110 } from '../batch-05/file-1110.js';
export interface Type1111 {
  id: 1111;
  name: 'File1111';
  next: Type1110;
}

export function make1111(): Type1111 {
  return { id: 1111, name: 'File1111', next: null as unknown as Type1110 };
}
