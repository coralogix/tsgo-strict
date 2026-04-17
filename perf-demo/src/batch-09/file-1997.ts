import type { Type1996 } from '../batch-09/file-1996.js';
export interface Type1997 {
  id: 1997;
  name: 'File1997';
  next: Type1996;
}

export function make1997(): Type1997 {
  return { id: 1997, name: 'File1997', next: null as unknown as Type1996 };
}
