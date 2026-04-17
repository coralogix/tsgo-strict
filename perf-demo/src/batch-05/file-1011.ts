import type { Type1010 } from '../batch-05/file-1010.js';
export interface Type1011 {
  id: 1011;
  name: 'File1011';
  next: Type1010;
}

export function make1011(): Type1011 {
  return { id: 1011, name: 'File1011', next: null as unknown as Type1010 };
}
