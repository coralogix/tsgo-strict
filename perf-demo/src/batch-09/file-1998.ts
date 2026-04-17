import type { Type1997 } from '../batch-09/file-1997.js';
export interface Type1998 {
  id: 1998;
  name: 'File1998';
  next: Type1997;
}

export function make1998(): Type1998 {
  return { id: 1998, name: 'File1998', next: null as unknown as Type1997 };
}
