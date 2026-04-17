import type { Type1809 } from '../batch-09/file-1809.js';
export interface Type1810 {
  id: 1810;
  name: 'File1810';
  next: Type1809;
}

export function make1810(): Type1810 {
  return { id: 1810, name: 'File1810', next: null as unknown as Type1809 };
}
