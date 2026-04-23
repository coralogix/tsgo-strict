import type { Type1128 } from '../batch-05/file-1128.js';
export interface Type1129 {
  id: 1129;
  name: 'File1129';
  next: Type1128;
}

export function make1129(): Type1129 {
  return { id: 1129, name: 'File1129', next: null as unknown as Type1128 };
}
