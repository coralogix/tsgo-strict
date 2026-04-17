import type { Type1035 } from '../batch-05/file-1035.js';
export interface Type1036 {
  id: 1036;
  name: 'File1036';
  next: Type1035;
}

export function make1036(): Type1036 {
  return { id: 1036, name: 'File1036', next: null as unknown as Type1035 };
}
