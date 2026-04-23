import type { Type1102 } from '../batch-05/file-1102.js';
export interface Type1103 {
  id: 1103;
  name: 'File1103';
  next: Type1102;
}

export function make1103(): Type1103 {
  return { id: 1103, name: 'File1103', next: null as unknown as Type1102 };
}
