import type { Type1401 } from '../batch-07/file-1401.js';
export interface Type1402 {
  id: 1402;
  name: 'File1402';
  next: Type1401;
}

export function make1402(): Type1402 {
  return { id: 1402, name: 'File1402', next: null as unknown as Type1401 };
}
