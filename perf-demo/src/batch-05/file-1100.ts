import type { Type1099 } from '../batch-05/file-1099.js';
export interface Type1100 {
  id: 1100;
  name: 'File1100';
  next: Type1099;
}

export function make1100(): Type1100 {
  return { id: 1100, name: 'File1100', next: null as unknown as Type1099 };
}
