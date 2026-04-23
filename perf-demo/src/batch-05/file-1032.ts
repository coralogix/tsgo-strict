import type { Type1031 } from '../batch-05/file-1031.js';
export interface Type1032 {
  id: 1032;
  name: 'File1032';
  next: Type1031;
}

export function make1032(): Type1032 {
  return { id: 1032, name: 'File1032', next: null as unknown as Type1031 };
}
