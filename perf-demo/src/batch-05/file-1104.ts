import type { Type1103 } from '../batch-05/file-1103.js';
export interface Type1104 {
  id: 1104;
  name: 'File1104';
  next: Type1103;
}

export function make1104(): Type1104 {
  return { id: 1104, name: 'File1104', next: null as unknown as Type1103 };
}
