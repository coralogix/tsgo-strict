import type { Type1030 } from '../batch-05/file-1030.js';
export interface Type1031 {
  id: 1031;
  name: 'File1031';
  next: Type1030;
}

export function make1031(): Type1031 {
  return { id: 1031, name: 'File1031', next: null as unknown as Type1030 };
}
