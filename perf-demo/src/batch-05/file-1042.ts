import type { Type1041 } from '../batch-05/file-1041.js';
export interface Type1042 {
  id: 1042;
  name: 'File1042';
  next: Type1041;
}

export function make1042(): Type1042 {
  return { id: 1042, name: 'File1042', next: null as unknown as Type1041 };
}
