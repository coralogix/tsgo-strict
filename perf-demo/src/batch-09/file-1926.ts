import type { Type1925 } from '../batch-09/file-1925.js';
export interface Type1926 {
  id: 1926;
  name: 'File1926';
  next: Type1925;
}

export function make1926(): Type1926 {
  return { id: 1926, name: 'File1926', next: null as unknown as Type1925 };
}
