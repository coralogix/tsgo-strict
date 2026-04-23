import type { Type1082 } from '../batch-05/file-1082.js';
export interface Type1083 {
  id: 1083;
  name: 'File1083';
  next: Type1082;
}

export function make1083(): Type1083 {
  return { id: 1083, name: 'File1083', next: null as unknown as Type1082 };
}
