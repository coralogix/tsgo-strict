import type { Type3602 } from '../batch-18/file-3602.js';
export interface Type3603 {
  id: 3603;
  name: 'File3603';
  next: Type3602;
}

export function make3603(): Type3603 {
  return { id: 3603, name: 'File3603', next: null as unknown as Type3602 };
}
