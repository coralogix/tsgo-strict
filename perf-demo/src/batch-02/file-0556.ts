import type { Type555 } from '../batch-02/file-0555.js';
export interface Type556 {
  id: 556;
  name: 'File556';
  next: Type555;
}

export function make556(): Type556 {
  return { id: 556, name: 'File556', next: null as unknown as Type555 };
}
