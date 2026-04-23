import type { Type1945 } from '../batch-09/file-1945.js';
export interface Type1946 {
  id: 1946;
  name: 'File1946';
  next: Type1945;
}

export function make1946(): Type1946 {
  return { id: 1946, name: 'File1946', next: null as unknown as Type1945 };
}
