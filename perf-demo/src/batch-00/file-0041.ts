import type { Type40 } from '../batch-00/file-0040.js';
export interface Type41 {
  id: 41;
  name: 'File41';
  next: Type40;
}

export function make41(): Type41 {
  return { id: 41, name: 'File41', next: null as unknown as Type40 };
}
