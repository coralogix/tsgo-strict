import type { Type1640 } from '../batch-08/file-1640.js';
export interface Type1641 {
  id: 1641;
  name: 'File1641';
  next: Type1640;
}

export function make1641(): Type1641 {
  return { id: 1641, name: 'File1641', next: null as unknown as Type1640 };
}
