import type { Type1040 } from '../batch-05/file-1040.js';
export interface Type1041 {
  id: 1041;
  name: 'File1041';
  next: Type1040;
}

export function make1041(): Type1041 {
  return { id: 1041, name: 'File1041', next: null as unknown as Type1040 };
}
