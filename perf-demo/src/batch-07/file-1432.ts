import type { Type1431 } from '../batch-07/file-1431.js';
export interface Type1432 {
  id: 1432;
  name: 'File1432';
  next: Type1431;
}

export function make1432(): Type1432 {
  return { id: 1432, name: 'File1432', next: null as unknown as Type1431 };
}
