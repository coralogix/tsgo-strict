import type { Type1599 } from '../batch-07/file-1599.js';
export interface Type1600 {
  id: 1600;
  name: 'File1600';
  next: Type1599;
}

export function make1600(): Type1600 {
  return { id: 1600, name: 'File1600', next: null as unknown as Type1599 };
}
