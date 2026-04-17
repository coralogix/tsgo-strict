import type { Type1952 } from '../batch-09/file-1952.js';
export interface Type1953 {
  id: 1953;
  name: 'File1953';
  next: Type1952;
}

export function make1953(): Type1953 {
  return { id: 1953, name: 'File1953', next: null as unknown as Type1952 };
}
