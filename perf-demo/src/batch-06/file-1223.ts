import type { Type1222 } from '../batch-06/file-1222.js';
export interface Type1223 {
  id: 1223;
  name: 'File1223';
  next: Type1222;
}

export function make1223(): Type1223 {
  return { id: 1223, name: 'File1223', next: null as unknown as Type1222 };
}
