import type { Type1051 } from '../batch-05/file-1051.js';
export interface Type1052 {
  id: 1052;
  name: 'File1052';
  next: Type1051;
}

export function make1052(): Type1052 {
  return { id: 1052, name: 'File1052', next: null as unknown as Type1051 };
}
