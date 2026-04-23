import type { Type1098 } from '../batch-05/file-1098.js';
export interface Type1099 {
  id: 1099;
  name: 'File1099';
  next: Type1098;
}

export function make1099(): Type1099 {
  return { id: 1099, name: 'File1099', next: null as unknown as Type1098 };
}
