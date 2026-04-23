import type { Type1050 } from '../batch-05/file-1050.js';
export interface Type1051 {
  id: 1051;
  name: 'File1051';
  next: Type1050;
}

export function make1051(): Type1051 {
  return { id: 1051, name: 'File1051', next: null as unknown as Type1050 };
}
