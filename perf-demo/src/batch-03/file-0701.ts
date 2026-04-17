import type { Type700 } from '../batch-03/file-0700.js';
export interface Type701 {
  id: 701;
  name: 'File701';
  next: Type700;
}

export function make701(): Type701 {
  return { id: 701, name: 'File701', next: null as unknown as Type700 };
}
