import type { Type1230 } from '../batch-06/file-1230.js';
export interface Type1231 {
  id: 1231;
  name: 'File1231';
  next: Type1230;
}

export function make1231(): Type1231 {
  return { id: 1231, name: 'File1231', next: null as unknown as Type1230 };
}
