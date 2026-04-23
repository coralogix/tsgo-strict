import type { Type2999 } from '../batch-14/file-2999.js';
export interface Type3000 {
  id: 3000;
  name: 'File3000';
  next: Type2999;
}

export function make3000(): Type3000 {
  return { id: 3000, name: 'File3000', next: null as unknown as Type2999 };
}
