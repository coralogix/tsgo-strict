import type { Type1882 } from '../batch-09/file-1882.js';
export interface Type1883 {
  id: 1883;
  name: 'File1883';
  next: Type1882;
}

export function make1883(): Type1883 {
  return { id: 1883, name: 'File1883', next: null as unknown as Type1882 };
}
