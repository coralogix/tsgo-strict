import type { Type1370 } from '../batch-06/file-1370.js';
export interface Type1371 {
  id: 1371;
  name: 'File1371';
  next: Type1370;
}

export function make1371(): Type1371 {
  return { id: 1371, name: 'File1371', next: null as unknown as Type1370 };
}
