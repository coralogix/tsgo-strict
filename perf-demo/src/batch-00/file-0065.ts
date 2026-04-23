import type { Type64 } from '../batch-00/file-0064.js';
export interface Type65 {
  id: 65;
  name: 'File65';
  next: Type64;
}

export function make65(): Type65 {
  return { id: 65, name: 'File65', next: null as unknown as Type64 };
}
