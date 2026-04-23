import type { Type2602 } from '../batch-13/file-2602.js';
export interface Type2603 {
  id: 2603;
  name: 'File2603';
  next: Type2602;
}

export function make2603(): Type2603 {
  return { id: 2603, name: 'File2603', next: null as unknown as Type2602 };
}
