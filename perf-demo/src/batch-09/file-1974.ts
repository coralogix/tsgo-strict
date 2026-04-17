import type { Type1973 } from '../batch-09/file-1973.js';
export interface Type1974 {
  id: 1974;
  name: 'File1974';
  next: Type1973;
}

export function make1974(): Type1974 {
  return { id: 1974, name: 'File1974', next: null as unknown as Type1973 };
}
