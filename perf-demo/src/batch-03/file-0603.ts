import type { Type602 } from '../batch-03/file-0602.js';
export interface Type603 {
  id: 603;
  name: 'File603';
  next: Type602;
}

export function make603(): Type603 {
  return { id: 603, name: 'File603', next: null as unknown as Type602 };
}
