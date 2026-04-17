import type { Type10 } from '../batch-00/file-0010.js';
export interface Type11 {
  id: 11;
  name: 'File11';
  next: Type10;
}

export function make11(): Type11 {
  return { id: 11, name: 'File11', next: null as unknown as Type10 };
}
