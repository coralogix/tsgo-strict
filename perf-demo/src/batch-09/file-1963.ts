import type { Type1962 } from '../batch-09/file-1962.js';
export interface Type1963 {
  id: 1963;
  name: 'File1963';
  next: Type1962;
}

export function make1963(): Type1963 {
  return { id: 1963, name: 'File1963', next: null as unknown as Type1962 };
}
