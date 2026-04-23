import type { Type1969 } from '../batch-09/file-1969.js';
export interface Type1970 {
  id: 1970;
  name: 'File1970';
  next: Type1969;
}

export function make1970(): Type1970 {
  return { id: 1970, name: 'File1970', next: null as unknown as Type1969 };
}
