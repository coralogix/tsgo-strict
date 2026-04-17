import type { Type1610 } from '../batch-08/file-1610.js';
export interface Type1611 {
  id: 1611;
  name: 'File1611';
  next: Type1610;
}

export function make1611(): Type1611 {
  return { id: 1611, name: 'File1611', next: null as unknown as Type1610 };
}
