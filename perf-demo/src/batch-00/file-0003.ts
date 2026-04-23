import type { Type2 } from '../batch-00/file-0002.js';
export interface Type3 {
  id: 3;
  name: 'File3';
  next: Type2;
}

export function make3(): Type3 {
  return { id: 3, name: 'File3', next: null as unknown as Type2 };
}
