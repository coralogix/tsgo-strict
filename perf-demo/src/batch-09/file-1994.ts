import type { Type1993 } from '../batch-09/file-1993.js';
export interface Type1994 {
  id: 1994;
  name: 'File1994';
  next: Type1993;
}

export function make1994(): Type1994 {
  return { id: 1994, name: 'File1994', next: null as unknown as Type1993 };
}
