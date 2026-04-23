import type { Type1911 } from '../batch-09/file-1911.js';
export interface Type1912 {
  id: 1912;
  name: 'File1912';
  next: Type1911;
}

export function make1912(): Type1912 {
  return { id: 1912, name: 'File1912', next: null as unknown as Type1911 };
}
