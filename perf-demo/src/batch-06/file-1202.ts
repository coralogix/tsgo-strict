import type { Type1201 } from '../batch-06/file-1201.js';
export interface Type1202 {
  id: 1202;
  name: 'File1202';
  next: Type1201;
}

export function make1202(): Type1202 {
  return { id: 1202, name: 'File1202', next: null as unknown as Type1201 };
}
