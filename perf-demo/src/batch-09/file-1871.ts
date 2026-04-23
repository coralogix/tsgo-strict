import type { Type1870 } from '../batch-09/file-1870.js';
export interface Type1871 {
  id: 1871;
  name: 'File1871';
  next: Type1870;
}

export function make1871(): Type1871 {
  return { id: 1871, name: 'File1871', next: null as unknown as Type1870 };
}
