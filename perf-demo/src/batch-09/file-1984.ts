import type { Type1983 } from '../batch-09/file-1983.js';
export interface Type1984 {
  id: 1984;
  name: 'File1984';
  next: Type1983;
}

export function make1984(): Type1984 {
  return { id: 1984, name: 'File1984', next: null as unknown as Type1983 };
}
