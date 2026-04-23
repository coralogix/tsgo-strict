import type { Type1611 } from '../batch-08/file-1611.js';
export interface Type1612 {
  id: 1612;
  name: 'File1612';
  next: Type1611;
}

export function make1612(): Type1612 {
  return { id: 1612, name: 'File1612', next: null as unknown as Type1611 };
}
