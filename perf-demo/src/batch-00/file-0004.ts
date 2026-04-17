import type { Type3 } from '../batch-00/file-0003.js';
export interface Type4 {
  id: 4;
  name: 'File4';
  next: Type3;
}

export function make4(): Type4 {
  return { id: 4, name: 'File4', next: null as unknown as Type3 };
}
