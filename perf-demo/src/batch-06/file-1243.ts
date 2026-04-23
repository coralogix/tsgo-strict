import type { Type1242 } from '../batch-06/file-1242.js';
export interface Type1243 {
  id: 1243;
  name: 'File1243';
  next: Type1242;
}

export function make1243(): Type1243 {
  return { id: 1243, name: 'File1243', next: null as unknown as Type1242 };
}
