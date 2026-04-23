import type { Type1963 } from '../batch-09/file-1963.js';
export interface Type1964 {
  id: 1964;
  name: 'File1964';
  next: Type1963;
}

export function make1964(): Type1964 {
  return { id: 1964, name: 'File1964', next: null as unknown as Type1963 };
}
