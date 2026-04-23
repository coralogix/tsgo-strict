import type { Type2042 } from '../batch-10/file-2042.js';
export interface Type2043 {
  id: 2043;
  name: 'File2043';
  next: Type2042;
}

export function make2043(): Type2043 {
  return { id: 2043, name: 'File2043', next: null as unknown as Type2042 };
}
