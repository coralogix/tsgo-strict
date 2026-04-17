import type { Type1980 } from '../batch-09/file-1980.js';
export interface Type1981 {
  id: 1981;
  name: 'File1981';
  next: Type1980;
}

export function make1981(): Type1981 {
  return { id: 1981, name: 'File1981', next: null as unknown as Type1980 };
}
