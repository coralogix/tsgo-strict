import type { Type1582 } from '../batch-07/file-1582.js';
export interface Type1583 {
  id: 1583;
  name: 'File1583';
  next: Type1582;
}

export function make1583(): Type1583 {
  return { id: 1583, name: 'File1583', next: null as unknown as Type1582 };
}
