import type { Type1901 } from '../batch-09/file-1901.js';
export interface Type1902 {
  id: 1902;
  name: 'File1902';
  next: Type1901;
}

export function make1902(): Type1902 {
  return { id: 1902, name: 'File1902', next: null as unknown as Type1901 };
}
