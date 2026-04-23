import type { Type1704 } from '../batch-08/file-1704.js';
export interface Type1705 {
  id: 1705;
  name: 'File1705';
  next: Type1704;
}

export function make1705(): Type1705 {
  return { id: 1705, name: 'File1705', next: null as unknown as Type1704 };
}
