import type { Type360 } from '../batch-01/file-0360.js';
export interface Type361 {
  id: 361;
  name: 'File361';
  next: Type360;
}

export function make361(): Type361 {
  return { id: 361, name: 'File361', next: null as unknown as Type360 };
}
