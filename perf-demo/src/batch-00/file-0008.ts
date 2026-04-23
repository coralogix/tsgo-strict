import type { Type7 } from '../batch-00/file-0007.js';
export interface Type8 {
  id: 8;
  name: 'File8';
  next: Type7;
}

export function make8(): Type8 {
  return { id: 8, name: 'File8', next: null as unknown as Type7 };
}
