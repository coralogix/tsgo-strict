import type { Type1064 } from '../batch-05/file-1064.js';
export interface Type1065 {
  id: 1065;
  name: 'File1065';
  next: Type1064;
}

export function make1065(): Type1065 {
  return { id: 1065, name: 'File1065', next: null as unknown as Type1064 };
}
