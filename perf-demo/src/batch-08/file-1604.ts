import type { Type1603 } from '../batch-08/file-1603.js';
export interface Type1604 {
  id: 1604;
  name: 'File1604';
  next: Type1603;
}

export function make1604(): Type1604 {
  return { id: 1604, name: 'File1604', next: null as unknown as Type1603 };
}
