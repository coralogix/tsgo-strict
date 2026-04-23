import type { Type1109 } from '../batch-05/file-1109.js';
export interface Type1110 {
  id: 1110;
  name: 'File1110';
  next: Type1109;
}

export function make1110(): Type1110 {
  return { id: 1110, name: 'File1110', next: null as unknown as Type1109 };
}
