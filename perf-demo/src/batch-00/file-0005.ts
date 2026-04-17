import type { Type4 } from '../batch-00/file-0004.js';
export interface Type5 {
  id: 5;
  name: 'File5';
  next: Type4;
}

export function make5(): Type5 {
  return { id: 5, name: 'File5', next: null as unknown as Type4 };
}
