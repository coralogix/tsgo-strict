import type { Type1979 } from '../batch-09/file-1979.js';
export interface Type1980 {
  id: 1980;
  name: 'File1980';
  next: Type1979;
}

export function make1980(): Type1980 {
  return { id: 1980, name: 'File1980', next: null as unknown as Type1979 };
}
