import type { Type1132 } from '../batch-05/file-1132.js';
export interface Type1133 {
  id: 1133;
  name: 'File1133';
  next: Type1132;
}

export function make1133(): Type1133 {
  return { id: 1133, name: 'File1133', next: null as unknown as Type1132 };
}
