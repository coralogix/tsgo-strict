import type { Type1700 } from '../batch-08/file-1700.js';
export interface Type1701 {
  id: 1701;
  name: 'File1701';
  next: Type1700;
}

export function make1701(): Type1701 {
  return { id: 1701, name: 'File1701', next: null as unknown as Type1700 };
}
