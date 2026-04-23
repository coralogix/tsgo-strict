import type { Type1360 } from '../batch-06/file-1360.js';
export interface Type1361 {
  id: 1361;
  name: 'File1361';
  next: Type1360;
}

export function make1361(): Type1361 {
  return { id: 1361, name: 'File1361', next: null as unknown as Type1360 };
}
