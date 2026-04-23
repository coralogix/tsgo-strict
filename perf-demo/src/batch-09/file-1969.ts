import type { Type1968 } from '../batch-09/file-1968.js';
export interface Type1969 {
  id: 1969;
  name: 'File1969';
  next: Type1968;
}

export function make1969(): Type1969 {
  return { id: 1969, name: 'File1969', next: null as unknown as Type1968 };
}
