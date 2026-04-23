import type { Type1232 } from '../batch-06/file-1232.js';
export interface Type1233 {
  id: 1233;
  name: 'File1233';
  next: Type1232;
}

export function make1233(): Type1233 {
  return { id: 1233, name: 'File1233', next: null as unknown as Type1232 };
}
