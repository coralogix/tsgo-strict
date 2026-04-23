import type { Type1881 } from '../batch-09/file-1881.js';
export interface Type1882 {
  id: 1882;
  name: 'File1882';
  next: Type1881;
}

export function make1882(): Type1882 {
  return { id: 1882, name: 'File1882', next: null as unknown as Type1881 };
}
