import type { Type1991 } from '../batch-09/file-1991.js';
export interface Type1992 {
  id: 1992;
  name: 'File1992';
  next: Type1991;
}

export function make1992(): Type1992 {
  return { id: 1992, name: 'File1992', next: null as unknown as Type1991 };
}
