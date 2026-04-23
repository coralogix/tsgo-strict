import type { Type1532 } from '../batch-07/file-1532.js';
export interface Type1533 {
  id: 1533;
  name: 'File1533';
  next: Type1532;
}

export function make1533(): Type1533 {
  return { id: 1533, name: 'File1533', next: null as unknown as Type1532 };
}
