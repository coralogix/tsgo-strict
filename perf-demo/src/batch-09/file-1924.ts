import type { Type1923 } from '../batch-09/file-1923.js';
export interface Type1924 {
  id: 1924;
  name: 'File1924';
  next: Type1923;
}

export function make1924(): Type1924 {
  return { id: 1924, name: 'File1924', next: null as unknown as Type1923 };
}
