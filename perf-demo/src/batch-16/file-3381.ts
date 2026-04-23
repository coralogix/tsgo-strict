import type { Type3380 } from '../batch-16/file-3380.js';
export interface Type3381 {
  id: 3381;
  name: 'File3381';
  next: Type3380;
}

export function make3381(): Type3381 {
  return { id: 3381, name: 'File3381', next: null as unknown as Type3380 };
}
