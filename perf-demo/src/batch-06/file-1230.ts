import type { Type1229 } from '../batch-06/file-1229.js';
export interface Type1230 {
  id: 1230;
  name: 'File1230';
  next: Type1229;
}

export function make1230(): Type1230 {
  return { id: 1230, name: 'File1230', next: null as unknown as Type1229 };
}
