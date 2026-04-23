import type { Type1042 } from '../batch-05/file-1042.js';
export interface Type1043 {
  id: 1043;
  name: 'File1043';
  next: Type1042;
}

export function make1043(): Type1043 {
  return { id: 1043, name: 'File1043', next: null as unknown as Type1042 };
}
