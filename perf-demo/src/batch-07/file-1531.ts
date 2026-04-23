import type { Type1530 } from '../batch-07/file-1530.js';
export interface Type1531 {
  id: 1531;
  name: 'File1531';
  next: Type1530;
}

export function make1531(): Type1531 {
  return { id: 1531, name: 'File1531', next: null as unknown as Type1530 };
}
