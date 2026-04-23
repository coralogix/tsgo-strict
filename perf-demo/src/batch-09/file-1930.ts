import type { Type1929 } from '../batch-09/file-1929.js';
export interface Type1930 {
  id: 1930;
  name: 'File1930';
  next: Type1929;
}

export function make1930(): Type1930 {
  return { id: 1930, name: 'File1930', next: null as unknown as Type1929 };
}
