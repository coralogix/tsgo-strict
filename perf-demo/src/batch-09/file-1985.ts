import type { Type1984 } from '../batch-09/file-1984.js';
export interface Type1985 {
  id: 1985;
  name: 'File1985';
  next: Type1984;
}

export function make1985(): Type1985 {
  return { id: 1985, name: 'File1985', next: null as unknown as Type1984 };
}
