import type { Type6 } from '../batch-00/file-0006.js';
export interface Type7 {
  id: 7;
  name: 'File7';
  next: Type6;
}

export function make7(): Type7 {
  return { id: 7, name: 'File7', next: null as unknown as Type6 };
}
