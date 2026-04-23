import type { Type1120 } from '../batch-05/file-1120.js';
export interface Type1121 {
  id: 1121;
  name: 'File1121';
  next: Type1120;
}

export function make1121(): Type1121 {
  return { id: 1121, name: 'File1121', next: null as unknown as Type1120 };
}
