import type { Type3100 } from '../batch-15/file-3100.js';
export interface Type3101 {
  id: 3101;
  name: 'File3101';
  next: Type3100;
}

export function make3101(): Type3101 {
  return { id: 3101, name: 'File3101', next: null as unknown as Type3100 };
}
