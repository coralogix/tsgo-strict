import type { Type1009 } from '../batch-05/file-1009.js';
export interface Type1010 {
  id: 1010;
  name: 'File1010';
  next: Type1009;
}

export function make1010(): Type1010 {
  return { id: 1010, name: 'File1010', next: null as unknown as Type1009 };
}
