import type { Type1233 } from '../batch-06/file-1233.js';
export interface Type1234 {
  id: 1234;
  name: 'File1234';
  next: Type1233;
}

export function make1234(): Type1234 {
  return { id: 1234, name: 'File1234', next: null as unknown as Type1233 };
}
