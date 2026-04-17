import type { Type11 } from '../batch-00/file-0011.js';
export interface Type12 {
  id: 12;
  name: 'File12';
  next: Type11;
}

export function make12(): Type12 {
  return { id: 12, name: 'File12', next: null as unknown as Type11 };
}
