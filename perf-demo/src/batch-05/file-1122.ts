import type { Type1121 } from '../batch-05/file-1121.js';
export interface Type1122 {
  id: 1122;
  name: 'File1122';
  next: Type1121;
}

export function make1122(): Type1122 {
  return { id: 1122, name: 'File1122', next: null as unknown as Type1121 };
}
