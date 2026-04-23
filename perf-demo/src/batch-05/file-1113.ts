import type { Type1112 } from '../batch-05/file-1112.js';
export interface Type1113 {
  id: 1113;
  name: 'File1113';
  next: Type1112;
}

export function make1113(): Type1113 {
  return { id: 1113, name: 'File1113', next: null as unknown as Type1112 };
}
