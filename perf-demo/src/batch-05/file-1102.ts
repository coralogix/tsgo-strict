import type { Type1101 } from '../batch-05/file-1101.js';
export interface Type1102 {
  id: 1102;
  name: 'File1102';
  next: Type1101;
}

export function make1102(): Type1102 {
  return { id: 1102, name: 'File1102', next: null as unknown as Type1101 };
}
