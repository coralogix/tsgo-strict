import type { Type1985 } from '../batch-09/file-1985.js';
export interface Type1986 {
  id: 1986;
  name: 'File1986';
  next: Type1985;
}

export function make1986(): Type1986 {
  return { id: 1986, name: 'File1986', next: null as unknown as Type1985 };
}
