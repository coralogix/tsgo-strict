import type { Type1140 } from '../batch-05/file-1140.js';
export interface Type1141 {
  id: 1141;
  name: 'File1141';
  next: Type1140;
}

export function make1141(): Type1141 {
  return { id: 1141, name: 'File1141', next: null as unknown as Type1140 };
}
