import type { Type1223 } from '../batch-06/file-1223.js';
export interface Type1224 {
  id: 1224;
  name: 'File1224';
  next: Type1223;
}

export function make1224(): Type1224 {
  return { id: 1224, name: 'File1224', next: null as unknown as Type1223 };
}
