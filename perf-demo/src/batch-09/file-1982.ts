import type { Type1981 } from '../batch-09/file-1981.js';
export interface Type1982 {
  id: 1982;
  name: 'File1982';
  next: Type1981;
}

export function make1982(): Type1982 {
  return { id: 1982, name: 'File1982', next: null as unknown as Type1981 };
}
