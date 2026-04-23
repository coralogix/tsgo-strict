import type { Type1131 } from '../batch-05/file-1131.js';
export interface Type1132 {
  id: 1132;
  name: 'File1132';
  next: Type1131;
}

export function make1132(): Type1132 {
  return { id: 1132, name: 'File1132', next: null as unknown as Type1131 };
}
