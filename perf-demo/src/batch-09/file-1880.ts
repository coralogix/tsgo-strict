import type { Type1879 } from '../batch-09/file-1879.js';
export interface Type1880 {
  id: 1880;
  name: 'File1880';
  next: Type1879;
}

export function make1880(): Type1880 {
  return { id: 1880, name: 'File1880', next: null as unknown as Type1879 };
}
