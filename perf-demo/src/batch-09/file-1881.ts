import type { Type1880 } from '../batch-09/file-1880.js';
export interface Type1881 {
  id: 1881;
  name: 'File1881';
  next: Type1880;
}

export function make1881(): Type1881 {
  return { id: 1881, name: 'File1881', next: null as unknown as Type1880 };
}
