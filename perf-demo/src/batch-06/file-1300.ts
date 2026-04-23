import type { Type1299 } from '../batch-06/file-1299.js';
export interface Type1300 {
  id: 1300;
  name: 'File1300';
  next: Type1299;
}

export function make1300(): Type1300 {
  return { id: 1300, name: 'File1300', next: null as unknown as Type1299 };
}
