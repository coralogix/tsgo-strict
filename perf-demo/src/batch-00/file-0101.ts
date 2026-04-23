import type { Type100 } from '../batch-00/file-0100.js';
export interface Type101 {
  id: 101;
  name: 'File101';
  next: Type100;
}

export function make101(): Type101 {
  return { id: 101, name: 'File101', next: null as unknown as Type100 };
}
