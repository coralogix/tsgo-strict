import type { Type1602 } from '../batch-08/file-1602.js';
export interface Type1603 {
  id: 1603;
  name: 'File1603';
  next: Type1602;
}

export function make1603(): Type1603 {
  return { id: 1603, name: 'File1603', next: null as unknown as Type1602 };
}
