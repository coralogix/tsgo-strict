import type { Type1380 } from '../batch-06/file-1380.js';
export interface Type1381 {
  id: 1381;
  name: 'File1381';
  next: Type1380;
}

export function make1381(): Type1381 {
  return { id: 1381, name: 'File1381', next: null as unknown as Type1380 };
}
