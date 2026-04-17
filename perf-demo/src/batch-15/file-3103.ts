import type { Type3102 } from '../batch-15/file-3102.js';
export interface Type3103 {
  id: 3103;
  name: 'File3103';
  next: Type3102;
}

export function make3103(): Type3103 {
  return { id: 3103, name: 'File3103', next: null as unknown as Type3102 };
}
